use core::fmt::Write;
use hecs::*;
use macroquad::prelude::*;
use macroquad::prelude::{clear_background, next_frame};
use std::fmt::Debug;

mod levels;

#[derive(Clone, Debug)]
struct Thing {
    rect: Rect,
    velocity: Vec2,
    acceleration: Vec2,
    color: Color,
    /// If this is 1.0 that means there's no friction
    friction: f32,
}

trait QuickFormat: Debug {
    fn append(&self, s: &mut String);
}

impl QuickFormat for Thing {
    fn append(&self, s: &mut String) {
        write!(s, "Thing {{rect:").unwrap();
        self.rect.append(s);
        write!(s, ",velocity:").unwrap();
        self.velocity.append(s);
        write!(s, ",acceleration:").unwrap();
        self.acceleration.append(s);
        write!(s, ",color:").unwrap();
        self.color.append(s);
        write!(s, ",friction:").unwrap();
        self.friction.append(s);
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

struct Player {}

struct Missile {}

#[derive(Debug)]
struct Level {
    things: Vec<Thing>,
}

#[macroquad::main("Sub")]
async fn main() {
    // let font = load_ttf_font("assets/OrelegaOne-Regular.ttf").await;

    let gravity_out_of_water = 0.2;

    let water_level = 300.;

    let max_color_lerp_depth = 3000.;

    let min_camera_zoom = 0.5;
    let max_camera_zoom = 2.0;
    let mut camera_focal_y = screen_height() / 2.0;
    let main_area_width = 570.;

    // let mut entities = Entities::new();

    // let mut missiles: Vec<EntityHandle> = Vec::new();

    let mut world = World::new();

    let mut level = levels::get_level();

    let setup = |level: &Level, world: &mut World| -> Entity {
        for thing in &level.things {
            let mut thing = thing.clone();
            world.spawn((thing,));
        }

        world.spawn((
            Player {},
            Thing {
                rect: Rect::new(0., water_level + 3000., 30., 15.),
                velocity: Vec2::new(0.02, 0.02),
                acceleration: Vec2::ZERO,
                color: RED,
                friction: 0.985,
            },
        ))
    };
    let mut player_entity = setup(&level, &mut world);
    let missile_size = Vec2::new(20., 10.);
    let mut entities_to_despawn = Vec::new();

    let mut in_level_editor = false;
    let mut editor_mode = 1;

    let mut camera_zoom = 1.0;

    let mut editor_start_drag: Option<Rect> = None;

    let ui = macroquad::ui::Ui::new(&mut miniquad::graphics::Context::new());

    loop {
        let left_wall = -main_area_width / 2.;
        let right_wall = main_area_width / 2.;

        let screen_width = screen_width();
        let screen_height = screen_height();

        // Toggle the level editor
        if is_key_pressed(KeyCode::E) {
            in_level_editor = !in_level_editor;
            if in_level_editor {
                info!("EDITOR ENABLED");
                world.clear();
                player_entity = setup(&level, &mut world);
                info!("LEVEL THINGS: {:?}", level.things.len());
            } else {
                info!("EDITOR DISABLED");
                info!("LEVEL THINGS: {:?}", level.things.len());
            }
        }

        if !in_level_editor {
            // ================= UPDATE =========================

            // Update missiles
            {
                let mut missiles = world.query::<(&Missile, &Thing)>();

                // Check if missiles are hitting anything
                'outer_for: for (missile_entity, (_missile, missile_thing)) in missiles.iter() {
                    if missile_thing.rect.left() < left_wall {
                        entities_to_despawn.push(missile_entity);
                        break 'outer_for;
                    }

                    if missile_thing.rect.x + missile_thing.rect.w > right_wall {
                        entities_to_despawn.push(missile_entity);
                        break 'outer_for;
                    }

                    let mut things = world.query::<(&Thing,)>();
                    for (thing_entity, (thing,)) in things.iter() {
                        if missile_entity != thing_entity && thing_entity != player_entity {
                            // Check for collision
                            if thing.rect.overlaps(&missile_thing.rect) {
                                // Collides!
                                entities_to_despawn.push(missile_entity);
                                break 'outer_for;
                            }
                        }
                    }
                }
            }

            for entity in entities_to_despawn.drain(..) {
                world.despawn(entity).unwrap();
            }

            // Move entities
            for (_, (thing,)) in world.query_mut::<(&mut Thing,)>() {
                thing.velocity += thing.acceleration;
                thing.velocity *= thing.friction;
                thing.rect = thing.rect.offset(thing.velocity);
            }

            let mut missile_to_spawn = None;
            {
                let mut query = world
                    .query_one::<(&Player, &mut Thing)>(player_entity)
                    .unwrap();
                let (_player, player_thing) = query.get().unwrap();
                player_thing.rect = player_thing.rect.offset(player_thing.velocity);

                // Player controls
                if player_thing.rect.bottom() < water_level {
                    player_thing.velocity.y += gravity_out_of_water;
                } else {
                    let player_acceleration = 0.1;
                    if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
                        player_thing.velocity.x -= player_acceleration;
                    }
                    if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
                        player_thing.velocity.x += player_acceleration;
                    }
                    if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
                        player_thing.velocity.y -= player_acceleration;
                    }
                    if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
                        player_thing.velocity.y += player_acceleration;
                    }
                    player_thing.velocity *= 0.985;
                }

                // Check the player against walls
                if player_thing.rect.left() < left_wall {
                    player_thing.rect.x = left_wall;
                    player_thing.velocity.x = 0.;
                }

                if player_thing.rect.x + player_thing.rect.w > right_wall {
                    player_thing.rect.x = right_wall - player_thing.rect.w;
                    player_thing.velocity.x = 0.;
                }

                // Zoom the camera as the player goes deeper at the start.
                let depth_lerp = ((player_thing.rect.y - water_level) / max_color_lerp_depth)
                    .min(1.0)
                    .max(0.0);

                camera_zoom = (max_camera_zoom - min_camera_zoom) * depth_lerp + min_camera_zoom;

                let camera_buffer = (screen_height / camera_zoom) * 2.0 * 0.35;
                if player_thing.rect.y > camera_focal_y + camera_buffer {
                    camera_focal_y = player_thing.rect.y - camera_buffer;
                }

                set_camera(&Camera2D {
                    target: vec2(0., camera_focal_y),
                    zoom: Vec2::new(camera_zoom / screen_width, -camera_zoom / screen_height),
                    ..Default::default()
                });

                // Fire missile
                if is_key_pressed(KeyCode::Space) {
                    let new_position = (player_thing.rect.point()
                        + Vec2::new(player_thing.rect.w / 2., player_thing.rect.h))
                        + Vec2::new(-missile_size.x / 2., missile_size.y / 2.);

                    let acceleration_direction =
                        Vec2::new(player_thing.velocity.x, 0.).normalize() * 0.1;
                    player_thing.velocity -= acceleration_direction * 20.;
                    missile_to_spawn = Some(Thing {
                        rect: Rect {
                            x: new_position.x,
                            y: new_position.y,
                            w: missile_size.x,
                            h: missile_size.y,
                        },
                        velocity: player_thing.velocity * 0.8 + acceleration_direction * 10.,
                        acceleration: acceleration_direction,
                        color: BLUE,
                        friction: 1.0,
                    });
                }
            }

            // Separated out to avoid borrow checker complaints.
            if let Some(missile_to_spawn) = missile_to_spawn.take() {
                world.spawn((missile_to_spawn, Missile {}));
            }
        } else {
            // ===================== LEVEL EDITOR =========================

            let camera = &Camera2D {
                target: vec2(0., camera_focal_y),
                zoom: Vec2::new(camera_zoom / screen_width, -camera_zoom / screen_height),
                ..Default::default()
            };
            set_camera(camera);

            camera_zoom += mouse_wheel().1 * 0.01;

            if is_key_pressed(KeyCode::Key1) {
                info!("PLAYER MOVE MODE");
                editor_mode = 1;
            }

            if is_key_pressed(KeyCode::Key2) {
                info!("BOX DRAW MODE");
                editor_mode = 2;
            }
            let camera_position = camera.screen_to_world(mouse_position().into());

            let mut level_edited = false;
            match editor_mode {
                1 => {
                    if is_mouse_button_pressed(MouseButton::Left) {
                        let mut query = world
                            .query_one::<(&Player, &mut Thing)>(player_entity)
                            .unwrap();
                        let (_, player_thing) = query.get().unwrap();
                        player_thing.rect.x = camera_position.x;
                        player_thing.rect.y = camera_position.y;
                        camera_focal_y = camera_position.y;
                    }
                }
                2 => {
                    if let Some(rect) = editor_start_drag.as_mut() {
                        rect.w = camera_position.x - rect.x;
                        rect.h = camera_position.y - rect.y;

                        if is_mouse_button_released(MouseButton::Left) {
                            let thing = Thing {
                                rect: *rect,
                                velocity: Vec2::ZERO,
                                acceleration: Vec2::ZERO,
                                color: BLUE,
                                friction: 0.0,
                            };
                            level.things.push(thing.clone());
                            info!("LEVEL THINGS: {:?}", level.things.len());
                            world.spawn((thing,));
                            editor_start_drag = None;
                            level_edited = true;
                        }
                    } else {
                        if is_mouse_button_down(MouseButton::Left) {
                            editor_start_drag = Some(Rect {
                                x: camera_position.x,
                                y: camera_position.y,
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

        // Draw water
        draw_rectangle(
            -main_area_width / 2.,
            0.,
            main_area_width,
            8000.,
            Color::new(27. / 255., 66. / 255., 81. / 255., 1.),
        );

        // Draw sky
        draw_rectangle(
            -main_area_width / 2.,
            -3000.,
            main_area_width,
            water_level + 3000.,
            Color::new(106. / 255., 183. / 255., 206. / 255., 1.),
        );

        /*
        // Calculate color change to simulate less light.
        let player_color_vec = Vec3::new(player_color.r, player_color.g, player_color.b);
        let color_lerp = depth_lerp.min(0.3);
        let draw_color =
            (Vec3::new(BLUE.r, BLUE.g, BLUE.b) - player_color_vec) * color_lerp + player_color_vec;

        // Draw player
        draw_rectangle(
            player_rect.x,
            player_rect.y,
            player_rect.w,
            player_rect.h,
            Color::new(draw_color.x, draw_color.y, draw_color.z, 1.0 - color_lerp),
        );
        */

        // Draw entities
        for (_entity, (thing,)) in &mut world.query::<(&Thing,)>() {
            draw_rectangle(
                thing.rect.x,
                thing.rect.y,
                thing.rect.w,
                thing.rect.h,
                thing.color,
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
