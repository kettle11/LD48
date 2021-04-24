use macroquad::prelude::*;
use macroquad::prelude::{clear_background, next_frame};
#[macroquad::main("Sub")]
async fn main() {
    let font = load_ttf_font("assets/OrelegaOne-Regular.ttf").await;

    let mut player_velocity = Vec2::new(0., 0.);
    let player_acceleration = Vec2::new(0.1, 0.1);
    let gravity_out_of_water = 0.2;

    let water_level = 300.;
    let player_color = RED;

    let max_color_lerp_depth = 5000.;

    let min_camera_zoom = 0.5;
    let max_camera_zoom = 2.0;
    let mut camera_focal_y = screen_height() / 2.0;
    let main_area_width = 570.;

    let mut player_rect = Rect::new(0., water_level + 30., 30., 15.);

    loop {
        let screen_width = screen_width();
        let screen_height = screen_height();

        // UPDATE
        player_rect = player_rect.offset(player_velocity);
        if player_rect.bottom() < water_level {
            player_velocity.y += gravity_out_of_water;
        } else {
            if is_key_down(KeyCode::Left) {
                player_velocity.x -= player_acceleration.x;
            }
            if is_key_down(KeyCode::Right) {
                player_velocity.x += player_acceleration.x;
            }
            if is_key_down(KeyCode::Up) {
                player_velocity.y -= player_acceleration.y;
            }
            if is_key_down(KeyCode::Down) {
                player_velocity.y += player_acceleration.y;
            }
            player_velocity *= 0.985;
        }

        let center_x = screen_width / 2.;

        if player_rect.left() < -main_area_width / 2. {
            player_rect.x = -main_area_width / 2.;
            player_velocity.x = 0.;
        }

        if player_rect.x + player_rect.w > main_area_width / 2. {
            player_rect.x = main_area_width / 2. - player_rect.w;
            player_velocity.x = 0.;
        }

        let depth_lerp = ((player_rect.y - water_level) / max_color_lerp_depth)
            .min(1.0)
            .max(0.0);

        let camera_zoom = (max_camera_zoom - min_camera_zoom) * depth_lerp + min_camera_zoom;

        let camera_buffer = (screen_height / camera_zoom) * 2.0 * 0.35;
        if player_rect.y > camera_focal_y + camera_buffer {
            camera_focal_y = player_rect.y - camera_buffer;
        }

        // DRAW
        set_camera(&Camera2D {
            target: vec2(0., camera_focal_y),
            zoom: Vec2::new(camera_zoom / screen_width, -camera_zoom / screen_height),
            ..Default::default()
        });

        clear_background(BLACK);

        let screen_left = center_x - main_area_width / 2.;
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
