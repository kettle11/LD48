use core::fmt::Write;
use hecs::*;
use macroquad::prelude::*;
use macroquad::prelude::{clear_background, next_frame};
use std::cmp::Ordering;
use std::fmt::Debug;

mod physics;
use physics::*;
mod levels;

#[derive(Clone, Debug, PartialEq)]
enum ThingType {
    Player,
    Rock,
    Missile,
    Enemy,
}

enum Drawable {
    Rock,
    Player,
    Enemy,
    Missile,
}

#[derive(Clone, Debug)]
struct Thing {
    color: Color,
    /// If this is 1.0 that means there's no friction
    destructable: bool,
    physics_handle: PhysicsHandle,
    index_in_level: Option<usize>,
}

#[derive(Debug)]
struct LevelItem {
    position: Vec2,
    thing_type: ThingType,
    half_size: Vec2,
}

impl LevelItem {
    pub fn spawn(&self, physics: &mut Physics, world: &mut World, index_in_level: usize) {
        match self.thing_type {
            // Need to spawn things here
            ThingType::Rock => {
                let physics_handle = physics.push(PhysicsObject {
                    mass: f32::INFINITY,
                    position: self.position,
                    last_position: self.position,
                    collider: Collider::Rectangle {
                        half_width: self.half_size.x,
                        half_height: self.half_size.y,
                    },
                    gravity_multiplier: 0.0,
                    friction: 0.0,
                });
                world.spawn((
                    Thing {
                        color: BLACK,
                        destructable: false,
                        physics_handle,
                        index_in_level: Some(index_in_level),
                    },
                    Drawable::Rock,
                ));
            }
            ThingType::Enemy => {
                let physics_handle = physics.push(PhysicsObject {
                    mass: 2.0,
                    position: self.position,
                    last_position: self.position,
                    collider: Collider::Circle {
                        radius: self.half_size.x,
                    },
                    gravity_multiplier: 1.0,
                    friction: 0.9,
                });
                world.spawn((
                    Thing {
                        color: GREEN,
                        destructable: false,
                        physics_handle,
                        index_in_level: Some(index_in_level),
                    },
                    Drawable::Enemy,
                    Enemy {
                        acceleration_direction: -Vec2::X,
                    },
                ));
            }
            _ => unimplemented!(),
        }
    }
}

trait QuickFormat: Debug {
    fn append(&self, s: &mut String);
}

impl QuickFormat for LevelItem {
    fn append(&self, s: &mut String) {
        write!(s, "LevelItem {{position:").unwrap();
        self.position.append(s);
        write!(s, ",half_size:").unwrap();
        self.half_size.append(s);
        match self.thing_type {
            ThingType::Rock => {
                write!(s, ",thing_type:ThingType::Rock").unwrap();
            }
            ThingType::Enemy => {
                write!(s, ",thing_type:ThingType::Enemy").unwrap();
            }
            _ => unreachable!(),
        }
        write!(s, "}}").unwrap();
    }
}

impl QuickFormat for Rect {
    fn append(&self, s: &mut String) {
        write!(
            s,
            "Rect {{x: {:?}, y: {:?}, w: {:?}, h: {:?}}}",
            self.x, self.y, self.w, self.h
        )
        .unwrap();
    }
}

impl QuickFormat for Vec2 {
    fn append(&self, s: &mut String) {
        write!(s, "Vec2::new({:?}, {:?})", self.x, self.y,).unwrap();
    }
}

impl QuickFormat for f32 {
    fn append(&self, s: &mut String) {
        write!(s, "{:?}", self).unwrap();
    }
}

impl QuickFormat for bool {
    fn append(&self, s: &mut String) {
        write!(s, "{:?}", self).unwrap();
    }
}

impl QuickFormat for Color {
    fn append(&self, s: &mut String) {
        write!(
            s,
            "Color {{r: {:?}, g: {:?}, b: {:?}, a: {:?}}}",
            self.r, self.g, self.b, self.a
        )
        .unwrap();
    }
}

struct Player {
    direction_x: f32,
}
struct Enemy {
    acceleration_direction: Vec2,
}

struct Missile {}

/// Accelerates the direction it's facing.
struct Accelerator {
    amount: Vec2,
}

#[derive(Debug)]
struct Level {
    things: Vec<LevelItem>,
}

#[macroquad::main("Sub")]
async fn main() {
    let tooth_fish_texture = load_texture("assets/ToothFish.png").await.unwrap();
    // let font = load_ttf_font("assets/OrelegaOne-Regular.ttf").await;

    // let gravity_out_of_water = 0.2;

    let water_level = 300.;

    let max_color_lerp_depth = 3000.;
    //
    let min_camera_zoom = 1.3;
    let max_camera_zoom = 2.0;
    let mut camera_focal_y = screen_height() / 2.0;
    let main_area_width = 570.;

    let mut world = World::new();

    let mut level = levels::get_level();

    let player_half_width = 15.;
    let player_half_height = 7.5;
    let player_spawn_offset = 300.;
    let setup = |level: &Level, world: &mut World, physics: &mut Physics| -> Entity {
        world.clear();
        physics.clear();
        for (index_in_level, thing) in level.things.iter().enumerate() {
            thing.spawn(physics, world, index_in_level);
        }

        // Spawn player

        let physics_handle = physics.push(PhysicsObject::new(
            1.0,
            Vec2::new(0., water_level + player_spawn_offset),
            Collider::Rectangle {
                half_width: player_half_width / 2.,
                half_height: player_half_height / 2.,
            },
            1.0,
            0.98,
        ));

        world.spawn((
            Player { direction_x: 1.0 },
            Thing {
                color: RED,
                destructable: false,
                physics_handle: physics_handle,
                index_in_level: None,
            },
            Drawable::Player,
        ))
    };
    let mut physics = Physics::new(water_level);
    physics.gravity = 0.2;

    let mut player_entity = setup(&level, &mut world, &mut physics);
    let missile_size = Vec2::new(10., 5.);
    //let mut entities_to_despawn = Vec::new();

    let mut in_level_editor = false;
    let mut editor_mode = 1;

    let mut camera_zoom = 1.0;
    let mut editor_start_drag: Option<Rect> = None;
    let mut collision_responses: Vec<(Entity, Vec2)> = Vec::new();
    let mut acceleration_to_apply: Vec<(Entity, Vec2)> = Vec::new();

    loop {
        //  let left_wall = -main_area_width / 2.;
        //  let right_wall = main_area_width / 2.;

        let screen_width = screen_width();
        let screen_height = screen_height();

        let player_center = {
            let mut query = world
                .query_one::<(&Player, &mut Thing)>(player_entity)
                .unwrap();
            let (_player, player_thing) = query.get().unwrap();
            let p = physics.get(player_thing.physics_handle);
            p.position
        };

        // Toggle the level editor
        if is_key_pressed(KeyCode::E) {
            in_level_editor = !in_level_editor;
            if in_level_editor {
                info!("EDITOR ENABLED");
                player_entity = setup(&level, &mut world, &mut physics);
                info!("LEVEL THINGS: {:?}", level.things.len());
            } else {
                info!("EDITOR DISABLED");
                info!("LEVEL THINGS: {:?}", level.things.len());
            }
        }

        if !in_level_editor {
            // ================= UPDATE =========================

            // info!("PLAYER CENTER: {:?}", player_center);

            // Make enemies follow player
            for (_entity, (thing, enemy)) in &mut world.query::<(&Thing, &mut Enemy)>() {
                let p = physics.get_mut(thing.physics_handle);
                let diff = player_center - p.position;
                enemy.acceleration_direction = diff;
                let distance = diff.length();
                if distance < 800. {
                    p.apply_force(0.2 * diff.normalize())
                }
            }

            // Accelerate things
            for (_entity, (thing, accelerator)) in &mut world.query::<(&Thing, &Accelerator)>() {
                let p = physics.get_mut(thing.physics_handle);
                p.apply_force(accelerator.amount)
            }

            let mut missile_to_spawn = None;
            {
                let mut query = world
                    .query_one::<(&mut Player, &mut Thing)>(player_entity)
                    .unwrap();
                let (player, player_thing) = query.get().unwrap();

                let player_physics = physics.get_mut(player_thing.physics_handle);
                // Player controls
                let player_acceleration = 0.1;

                if player_center.y < water_level {
                    if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
                        player_physics.apply_force(Vec2::new(0., player_acceleration));
                    }
                } else {
                    if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
                        player_physics.apply_force(Vec2::new(-player_acceleration, 0.));
                        player.direction_x = -1.0;
                    }
                    if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
                        player_physics.apply_force(Vec2::new(player_acceleration, 0.));
                        player.direction_x = 1.0;
                    }
                    if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
                        player_physics.apply_force(Vec2::new(0., -player_acceleration));
                    }
                    if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
                        player_physics.apply_force(Vec2::new(0., player_acceleration));
                    }
                }

                // Check the player against walls
                /*
                if player_thing.rect.left() < left_wall {
                    player_thing.rect.x = left_wall;
                    player_thing.velocity.x = 0.;
                }

                if player_thing.rect.x + player_thing.rect.w > right_wall {
                    player_thing.rect.x = right_wall - player_thing.rect.w;
                    player_thing.velocity.x = 0.;
                }*/

                // Zoom the camera as the player goes deeper at the start.
                let depth_lerp = ((player_center.y - water_level) / max_color_lerp_depth)
                    .min(1.0)
                    .max(0.0);

                camera_zoom = (max_camera_zoom - min_camera_zoom) * depth_lerp + min_camera_zoom;

                let camera_buffer = (screen_height / camera_zoom) * 2.0 * 0.3;
                if player_center.y > camera_focal_y + camera_buffer {
                    camera_focal_y = player_center.y - camera_buffer;
                }

                if player_center.y < camera_focal_y - camera_buffer {
                    camera_focal_y = player_center.y + camera_buffer;
                }

                set_camera(&Camera2D {
                    target: vec2(0., camera_focal_y),
                    zoom: Vec2::new(camera_zoom / screen_width, -camera_zoom / screen_height),
                    ..Default::default()
                });

                // Fire missile
                if is_key_pressed(KeyCode::Space) {
                    let position = (player_physics.position
                        + Vec2::new(player_half_width, player_half_height))
                        + Vec2::new(-missile_size.x / 2., missile_size.y / 2.);

                    let acceleration_direction = Vec2::new(player.direction_x, 0.4) * 0.1;
                    player_physics.apply_force(-acceleration_direction * 10.);

                    let mut physics_object = PhysicsObject::new(
                        2.0,
                        position,
                        Collider::Rectangle {
                            half_width: missile_size.x,
                            half_height: missile_size.y,
                        },
                        1.0,
                        1.0,
                    );
                    physics_object.apply_force(acceleration_direction * 10.);
                    let physics_handle = physics.push(physics_object);
                    missile_to_spawn = Some((
                        Thing {
                            color: BLUE,
                            destructable: false,
                            physics_handle: physics_handle,
                            index_in_level: None,
                        },
                        player.direction_x,
                    ));
                }
            }

            // Separated out to avoid borrow checker complaints.

            if let Some((missile_to_spawn, direction)) = missile_to_spawn.take() {
                world.spawn((
                    missile_to_spawn,
                    Missile {},
                    Drawable::Missile,
                    Accelerator {
                        amount: Vec2::new(direction * 0.07, 0.0),
                    },
                ));
            }

            set_camera(&Camera2D {
                target: vec2(0., camera_focal_y),
                zoom: Vec2::new(camera_zoom / screen_width, -camera_zoom / screen_height),
                ..Default::default()
            });

            physics.run();
        } else {
            // ===================== LEVEL EDITOR =========================

            let camera = &Camera2D {
                target: vec2(0., camera_focal_y),
                zoom: Vec2::new(camera_zoom / screen_width, -camera_zoom / screen_height),
                ..Default::default()
            };
            set_camera(camera);

            camera_zoom += mouse_wheel().1 * 0.01;

            if is_key_pressed(KeyCode::Key0) {
                info!("ERASE MODE");
                editor_mode = 0;
            }

            if is_key_pressed(KeyCode::Key1) {
                info!("PLAYER MOVE MODE");
                editor_mode = 1;
            }

            if is_key_pressed(KeyCode::Key2) {
                info!("ROCK DRAW MODE");
                editor_mode = 2;
            }

            if is_key_pressed(KeyCode::Key3) {
                info!("ENEMY DRAW MODE");
                editor_mode = 3;
            }
            let mouse_position_in_world = camera.screen_to_world(mouse_position().into());

            match editor_mode {
                0 => {
                    if is_mouse_button_pressed(MouseButton::Left) {
                        let mut to_despawn = None;
                        for (entity, (thing,)) in world.query::<(&Thing,)>().iter() {
                            let p = physics.get(thing.physics_handle);
                            match p.collider {
                                Collider::Circle { radius } => {
                                    let distance = p.position - mouse_position_in_world;
                                    if distance.length() < radius {
                                        to_despawn = Some((entity, thing.index_in_level));
                                    }
                                }
                                Collider::Rectangle {
                                    half_width,
                                    half_height,
                                } => {
                                    let distance = (p.position - mouse_position_in_world).abs();
                                    if distance.x < half_width && distance.y < half_height {
                                        to_despawn = Some((entity, thing.index_in_level));
                                    }
                                }
                            }
                        }

                        if let Some((to_despawn, index)) = to_despawn {
                            if let Some(index) = index {
                                // We need to despawn in the physics, the level, and the ECS
                                let _ = world.despawn(to_despawn);
                                level.things.swap_remove(index);

                                // Just reload the entire level, but preserve the player position.
                                player_entity = setup(&level, &mut world, &mut physics);
                                let mut query = world
                                    .query_one::<(&mut Player, &mut Thing)>(player_entity)
                                    .unwrap();
                                let (_player, player_thing) = query.get().unwrap();
                                physics.get_mut(player_thing.physics_handle).position =
                                    player_center;
                            }
                        }
                    }
                }
                1 => {
                    if is_mouse_button_pressed(MouseButton::Left) {
                        let mut query = world
                            .query_one::<(&Player, &mut Thing)>(player_entity)
                            .unwrap();
                        let (_, player_thing) = query.get().unwrap();
                        let p = physics.get_mut(player_thing.physics_handle);
                        p.position = mouse_position_in_world;
                        p.last_position = mouse_position_in_world;
                        camera_focal_y = mouse_position_in_world.y;
                    }
                }
                2 | 3 => {
                    if let Some(rect) = editor_start_drag.as_mut() {
                        rect.w = mouse_position_in_world.x - rect.x;
                        rect.h = mouse_position_in_world.y - rect.y;

                        if is_mouse_button_released(MouseButton::Left) {
                            if rect.w < 0. {
                                rect.x = rect.right();
                                rect.w *= -1.
                            }
                            if rect.h < 0. {
                                rect.y = rect.bottom();
                                rect.h *= -1.
                            }
                            let level_item = match editor_mode {
                                2 => LevelItem {
                                    position: rect.point() + rect.size() / 2.,
                                    half_size: rect.size() / 2.,
                                    thing_type: ThingType::Rock,
                                },
                                3 => LevelItem {
                                    position: rect.point() + rect.size() / 2.,
                                    half_size: rect.size() / 2.,
                                    thing_type: ThingType::Enemy,
                                },
                                _ => unimplemented!(),
                            };
                            level_item.spawn(&mut physics, &mut world, level.things.len());
                            level.things.push(level_item);
                            editor_start_drag = None;
                        }
                    } else {
                        if is_mouse_button_down(MouseButton::Left) {
                            editor_start_drag = Some(Rect {
                                x: mouse_position_in_world.x,
                                y: mouse_position_in_world.y,
                                w: 1.,
                                h: 1.,
                            })
                        }
                    }
                }
                _ => {}
            }

            camera_zoom = camera_zoom.clamp(0.1, 10.);

            if is_key_pressed(KeyCode::P) {
                // Sort by top of thing
                level.things.sort_by(|a, b| {
                    (a.position.y - a.half_size.y)
                        .partial_cmp(&(b.position.y - b.half_size.y))
                        .unwrap_or(Ordering::Equal)
                });

                let mut s =
                    "use crate::*;\npub(crate) fn get_level() -> Level { Level {\n things: vec!["
                        .to_string();
                for i in &level.things {
                    i.append(&mut s);
                    s.push(',');
                }
                s.push_str("]}}");
                info!("{}", s);
            }
        }

        // ===================== DRAW =========================
        clear_background(BLACK);

        let above_water = player_center.y < water_level + 4.;
        if above_water {
            // Draw sky
            draw_rectangle(
                0. - screen_width / camera_zoom, //-main_area_width / 2.,
                -3000.,
                screen_width / camera_zoom * 2., //main_area_width,
                water_level + 3000.,
                Color::new(106. / 255., 183. / 255., 206. / 255., 1.),
            );
        } else {
            // Draw underwater background
            draw_rectangle(
                0. - screen_width / camera_zoom, //-main_area_width / 2.,
                -3000.,
                screen_width / camera_zoom * 2., //main_area_width,
                water_level + 3000.,
                Color::new(20. / 255., 50. / 255., 70. / 255., 1.),
            );

            draw_rectangle(
                0. - screen_width / camera_zoom, //-main_area_width / 2.,
                water_level,
                screen_width / camera_zoom * 2., // main_area_width,
                30000.,
                Color::new(27. / 255., 66. / 255., 81. / 255., 1.),
            );
        }

        /*
        // Calculate color change to simulate less light.
        let player_color_vec = Vec3::new(player_color.r, player_color.g, player_color.b);
        let color_lerp = depth_lerp.min(0.3);
        let draw_color =
            (Vec3::new(BLUE.r, BLUE.g, BLUE.b) - player_color_vec) * color_lerp + player_color_vec;
        */

        // Draw entities
        for (_entity, (thing, drawable, enemy)) in
            &mut world.query::<(&Thing, &Drawable, Option<&Enemy>)>()
        {
            let physics_object = physics.get(thing.physics_handle);

            match drawable {
                Drawable::Enemy => match physics_object.collider {
                    Collider::Circle { radius } => {
                        let direction = enemy.unwrap().acceleration_direction.normalize();
                        let rotation = direction.angle_between(-Vec2::X);

                        draw_texture_ex(
                            tooth_fish_texture,
                            physics_object.position.x - radius,
                            physics_object.position.y - radius,
                            WHITE,
                            DrawTextureParams {
                                dest_size: Some(Vec2::new(radius * 2., radius * 2.)),
                                rotation: -rotation,
                                ..Default::default()
                            },
                        );
                        /*
                        draw_circle(
                            physics_object.position.x,
                            physics_object.position.y,
                            radius,
                            thing.color,
                        );
                        */
                    }
                    _ => unimplemented!(),
                },
                _ => match physics_object.collider {
                    Collider::Rectangle {
                        half_width,
                        half_height,
                    } => {
                        draw_rectangle(
                            physics_object.position.x - half_width,
                            physics_object.position.y - half_height,
                            half_width * 2.,
                            half_height * 2.,
                            thing.color,
                        );
                    }
                    _ => unimplemented!(),
                },
            }
        }

        // Draw water here to draw over entities
        if above_water {
            // Draw water
            draw_rectangle(
                0. - screen_width / camera_zoom, //-main_area_width / 2.,
                water_level,
                screen_width / camera_zoom * 2., //main_area_width,
                400000.,
                Color::new(27. / 255., 66. / 255., 81. / 255., 1.),
            );
        }

        if let Some(rect) = editor_start_drag {
            draw_rectangle(rect.x, rect.y, rect.w, rect.h, BLUE);
        }

        /*
        draw_text_ex(
            "DEEP",
            -main_area_width / 2. + 100.,
            -500.,
            TextParams {
                font,
                color: Color::new(0.8, 0.28, 0.36, 1.0),
                font_size: 160,
                font_scale: 1.0,
                ..Default::default()
            },
        );
        */

        next_frame().await
    }
}
