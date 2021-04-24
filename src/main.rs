use hecs::*;
use macroquad::prelude::*;
use macroquad::prelude::{clear_background, next_frame};

#[derive(Clone, Debug)]
struct Thing {
    rect: Rect,
    velocity: Vec2,
    acceleration: Vec2,
    color: Color,
    /// If this is 1.0 that means there's no friction
    friction: f32,
}

struct Player {}

struct Missile {}

#[macroquad::main("Sub")]
async fn main() {
    let font = load_ttf_font("assets/OrelegaOne-Regular.ttf").await;

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

    let player_entity = world.spawn((
        Player {},
        Thing {
            rect: Rect::new(0., water_level + 3000., 30., 15.),
            velocity: Vec2::new(0.02, 0.02),
            acceleration: Vec2::ZERO,
            color: RED,
            friction: 0.985,
        },
    ));

    let missile_size = Vec2::new(20., 10.);
    let mut entities_to_despawn = Vec::new();

    loop {
        let left_wall = -main_area_width / 2.;
        let right_wall = main_area_width / 2.;

        let screen_width = screen_width();
        let screen_height = screen_height();

        // ================= UPDATE =========================

        // Missile logic
        for (entity, (_missile, thing)) in world.query::<(&Missile, &mut Thing)>().iter() {
            let mut explode_missile = false;
            if thing.rect.left() < left_wall {
                thing.rect.x = left_wall;
                thing.velocity.x = 0.;
                explode_missile = true;
            }

            if thing.rect.x + thing.rect.w > right_wall {
                thing.rect.x = right_wall - thing.rect.w;
                thing.velocity.x = 0.;
                explode_missile = true;
            }

            if explode_missile {
                entities_to_despawn.push(entity)
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
            let (player, player_thing) = query.get().unwrap();
            player_thing.rect = player_thing.rect.offset(player_thing.velocity);

            // Player controls
            if player_thing.rect.bottom() < water_level {
                player_thing.velocity.y += gravity_out_of_water;
            } else {
                let player_acceleration = 0.1;
                if is_key_down(KeyCode::Left) {
                    player_thing.velocity.x -= player_acceleration;
                }
                if is_key_down(KeyCode::Right) {
                    player_thing.velocity.x += player_acceleration;
                }
                if is_key_down(KeyCode::Up) {
                    player_thing.velocity.y -= player_acceleration;
                }
                if is_key_down(KeyCode::Down) {
                    player_thing.velocity.y += player_acceleration;
                }
                player_thing.velocity *= 0.985;
            }

            // Check the player against walls
            if player_thing.rect.left() < left_wall {
                player_thing.rect.x = left_wall;
                player_thing.velocity.x = 0.;

                info!("HITTING WALL LEFT: {:?}", player_thing.velocity);
            }

            if player_thing.rect.x + player_thing.rect.w > right_wall {
                player_thing.rect.x = right_wall - player_thing.rect.w;
                player_thing.velocity.x = 0.;
                info!("HITTING WALL RIGHT: {:?}", player_thing.velocity);
            }

            // Zoom the camera as the player goes deeper at the start.
            let depth_lerp = ((player_thing.rect.y - water_level) / max_color_lerp_depth)
                .min(1.0)
                .max(0.0);

            let camera_zoom = (max_camera_zoom - min_camera_zoom) * depth_lerp + min_camera_zoom;

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
        for (entity, (thing,)) in &mut world.query::<(&Thing,)>() {
            draw_rectangle(thing.rect.x, thing.rect.y, thing.rect.w, thing.rect.h, BLUE);
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
