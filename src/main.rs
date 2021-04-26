use core::fmt::Write;
use hecs::*;
use macroquad::{audio::*, prelude::*, rand::rand};
use macroquad::{
    prelude::{clear_background, next_frame},
    rand::gen_range,
};
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
    Checkpoint,
    TorpedoUpgrade,
}

enum Drawable {
    Rock,
    Player,
    Enemy,
    Missile,
    TorpedoUpgrade,
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

struct Checkpoint {
    position: Vec2,
    index_in_level: usize,
}

struct TorpedoUpgrade {
    position: Vec2,
    index_in_level: usize,
}

impl LevelItem {
    pub fn spawn(&self, physics: &mut Physics, world: &mut World, index_in_level: usize) {
        match self.thing_type {
            ThingType::Checkpoint => {
                world.spawn((
                    Checkpoint {
                        position: self.position,
                        index_in_level,
                    },
                    Drawable::Rock,
                ));
            }
            ThingType::TorpedoUpgrade => {
                world.spawn((
                    TorpedoUpgrade {
                        position: self.position,
                        index_in_level,
                    },
                    Drawable::TorpedoUpgrade,
                ));
            }
            ThingType::Rock => {
                let physics_handle = physics.push(PhysicsObject::new(
                    f32::INFINITY,
                    self.position,
                    Collider::Rectangle {
                        half_width: self.half_size.x,
                        half_height: self.half_size.y,
                    },
                    0.0,
                    0.0,
                ));
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
                let physics_handle = physics.push(PhysicsObject::new(
                    2.0,
                    self.position,
                    Collider::Circle {
                        radius: self.half_size.x,
                    },
                    1.0,
                    0.9,
                ));

                // Scale enemy health to their size.
                let health = (((self.half_size.x * self.half_size.x) / 800.) as u32).max(1);

                // info!("ENEMY HEALTH: {:?}", health);

                world.spawn((
                    Thing {
                        color: GREEN,
                        destructable: false,
                        physics_handle,
                        index_in_level: Some(index_in_level),
                    },
                    Drawable::Enemy,
                    Enemy {
                        following_player_sound_cooldown: 0,
                        hit_player_cooldown: 0,
                        acceleration_direction: -Vec2::X,
                    },
                    Health(health),
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
            ThingType::Checkpoint => {
                write!(s, ",thing_type:ThingType::Checkpoint").unwrap();
            }
            ThingType::TorpedoUpgrade => {
                write!(s, ",thing_type:ThingType::TorpedoUpgrade").unwrap();
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
    upgrades_collected: u32,
}

const HIT_PLAYER_COOLDOWN: u32 = 30;

const FOLLOWING_PLAYER_SOUND_COOLDOW: u32 = 120;
struct Health(u32);
struct Enemy {
    following_player_sound_cooldown: u32,
    hit_player_cooldown: u32,
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

struct ExplosionPiece {
    position: Vec2,
    radius: f32,
    color: Color,
}
struct Explosion {
    radius: f32,
    scale: f32,
    center: Vec2,
    pieces: Vec<ExplosionPiece>,
    damaged: bool,
    follow_player: bool,
}

impl Explosion {
    pub fn new(center: Vec2, radius: f32, scale: f32, follow_player: bool) -> Self {
        let number_of_pieces = rand::gen_range(4, 8);
        let pieces = Vec::with_capacity(number_of_pieces);
        let mut s = Self {
            radius,
            scale,
            center,
            pieces,
            damaged: false,
            follow_player,
        };
        for _ in 0..number_of_pieces {
            s.spawn_piece();
        }
        s
    }

    pub fn spawn_piece(&mut self) {
        let max_radius = 20.;

        let rand_x = gen_range(-max_radius, max_radius) * self.scale;
        let rand_y = gen_range(-max_radius, max_radius) * self.scale;

        let color = match gen_range(0, 3) {
            0 => Color::new(212. / 255., 39. / 255., 15. / 255., 1.0),
            1 => Color::new(239. / 255., 249. / 255., 126. / 255., 1.0),
            2 => Color::new(204. / 255., 137. / 255., 75. / 255., 1.0),
            _ => unreachable!(),
        };

        self.pieces.push(ExplosionPiece {
            position: self.center + Vec2::new(rand_x, rand_y),
            radius: gen_range(20., self.radius - max_radius) * self.scale,
            color,
        })
    }

    pub fn draw(&mut self, player_position: Vec2) {
        if self.follow_player {
            self.center = player_position;
        }
        if gen_range(0, 100) < 35 {
            self.spawn_piece()
        }

        if gen_range(0, 100) < 50 {
            if self.pieces.len() > 0 {
                self.pieces.swap_remove(0);
            }
        }

        for piece in &self.pieces {
            draw_poly(
                piece.position.x,
                piece.position.y,
                7,
                piece.radius,
                0.,
                piece.color,
            )
        }
    }
}

static mut UPGRADES_COLLECTED: u32 = 1;
const TORPEDO_UPGRADE_RADIUS: f32 = 35.;
#[macroquad::main("Sub")]
async fn main() {
    let explosion_sounds = [
        load_sound("assets/Explosion0.wav").await.unwrap(),
        load_sound("assets/Explosion1.wav").await.unwrap(),
        load_sound("assets/Explosion2.wav").await.unwrap(),
        load_sound("assets/Explosion4.wav").await.unwrap(),
    ];

    let enemy_follow_sounds = [
        load_sound("assets/enemy_follow.wav").await.unwrap(),
        load_sound("assets/enemy_notice.wav").await.unwrap(),
    ];

    let end_music = load_sound("assets/ld48_end.mp3").await.unwrap();
    let hello_sound = load_sound("assets/hello.wav").await.unwrap();
    let upgrade_sound = load_sound("assets/upgrade.wav").await.unwrap();
    let hit_sound = load_sound("assets/hurt.wav").await.unwrap();
    let enemy_death_sound = load_sound("assets/enemy_death.wav").await.unwrap();
    let shoot_sound = load_sound("assets/shoot.wav").await.unwrap();
    let dead_tooth_fish_texture = load_texture("assets/DeadToothFish.png").await.unwrap();
    let tooth_fish_texture = load_texture("assets/ToothFish.png").await.unwrap();

    let home_texture0 = load_texture("assets/home.png").await.unwrap();
    let home_texture1 = load_texture("assets/home2.png").await.unwrap();

    let mut home_texture = home_texture0;
    let torpedo_upgrade = load_texture("assets/torpedo_upgrade.png").await.unwrap();

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
    let player_spawn_offset = 0.;

    let mut screen_shake_amount: f32 = 0.;

    let player_start = Vec2::new(0., water_level + player_spawn_offset);
    let setup = |level: &Level,
                 world: &mut World,
                 physics: &mut Physics,
                 respawn_timer: &mut u32,
                 player_position: Vec2|
     -> Entity {
        world.clear();
        physics.clear();

        for (index_in_level, thing) in level.things.iter().enumerate() {
            thing.spawn(physics, world, index_in_level);
        }

        *respawn_timer = RESPAWN_TIMER;

        let mut physics_object = PhysicsObject::new(
            1.0,
            player_position,
            Collider::Rectangle {
                half_width: player_half_width / 2.,
                half_height: player_half_height / 2.,
            },
            1.0,
            0.98,
        );
        // Obviously the player is not a missile, but this flag makes
        // the player not explode missiles when they're bumped.
        physics_object.is_missile = true;

        // Spawn player
        let physics_handle = physics.push(physics_object);

        world.spawn((
            Player {
                direction_x: 1.0,
                upgrades_collected: 0,
            },
            Thing {
                color: RED,
                destructable: false,
                physics_handle: physics_handle,
                index_in_level: None,
            },
            // Player health
            Health(5),
            Drawable::Player,
        ))
    };
    let victory_depth = 34000.0;

    let mut physics = Physics::new(water_level, victory_depth);
    physics.gravity = 0.2;

    const RESPAWN_TIMER: u32 = 60 * 6;
    let mut respawn_timer = RESPAWN_TIMER;

    let mut player_entity = setup(
        &level,
        &mut world,
        &mut physics,
        &mut respawn_timer,
        player_start,
    );
    let missile_size = Vec2::new(8., 4.);
    let mut entities_to_despawn = Vec::new();

    let mut in_level_editor = false;
    let mut editor_mode = 1;

    let mut camera_zoom = 1.0;
    let mut editor_start_drag: Option<Rect> = None;
    let mut collision_responses: Vec<(Entity, Vec2)> = Vec::new();
    let mut acceleration_to_apply: Vec<(Entity, Vec2)> = Vec::new();

    let mut explosions_to_spawn = Vec::new();

    let mut camera_angle: f32 = 0.;
    let mut camera_target_angle: f32 = 0.;
    let mut camera_flip_counter: f32 = 1.;
    let mut camera_flipped = false;
    /*c
    // TEMPORARY
    world.spawn((Explosion::new(Vec2::new(
        0.,
        water_level + player_spawn_offset,
    )),));
    */

    let mut home_timer = 0;
    let mut home_frame = 0;
    loop {
        home_timer += 1;
        if home_timer > 33 {
            if home_frame == 0 {
                home_texture = home_texture1;
                home_timer = 0;
                home_frame = 1;
            } else {
                home_texture = home_texture0;
                home_timer = 0;
                home_frame = 0;
            }
        }

        screen_shake_amount *= 0.94;
        //  let left_wall = -main_area_width / 2.;
        //  let right_wall = main_area_width / 2.;

        let screen_width = screen_width();
        let screen_height = screen_height();

        let (player_center, player_physics_handle) = {
            let mut query = world
                .query_one::<(&Player, &mut Thing)>(player_entity)
                .unwrap();
            let (_player, player_thing) = query.get().unwrap();
            let p = physics.get(player_thing.physics_handle).unwrap();
            (p.position, player_thing.physics_handle)
        };

        let below_victory_depth = player_center.y > victory_depth - 4.;
        let above_water = player_center.y < water_level + 4.;

        // Toggle the level editor
        if is_key_pressed(KeyCode::E) {
            in_level_editor = !in_level_editor;
            if in_level_editor {
                info!("EDITOR ENABLED");
                player_entity = setup(
                    &level,
                    &mut world,
                    &mut physics,
                    &mut respawn_timer,
                    player_start,
                );
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
            for (_entity, (thing, enemy, health)) in
                &mut world.query::<(&Thing, &mut Enemy, &Health)>()
            {
                enemy.hit_player_cooldown = enemy.hit_player_cooldown.saturating_sub(1);
                let p = physics.get_mut(thing.physics_handle).unwrap();

                if health.0 > 0 {
                    enemy.following_player_sound_cooldown =
                        enemy.following_player_sound_cooldown.saturating_sub(1);

                    let diff = player_center - p.position;
                    enemy.acceleration_direction = diff;
                    let distance = diff.length();
                    if distance < 800. {
                        if enemy.following_player_sound_cooldown == 0 {
                            play_sound_once(
                                enemy_follow_sounds[rand::gen_range(0, enemy_follow_sounds.len())],
                            );

                            enemy.following_player_sound_cooldown = FOLLOWING_PLAYER_SOUND_COOLDOW;
                        }

                        p.apply_force(0.2 * diff.normalize());

                        let radius = match p.collider {
                            Collider::Circle { radius } => radius,
                            _ => unimplemented!(),
                        };

                        // Player damage checks
                        if enemy.hit_player_cooldown == 0
                            && distance < player_half_height + radius + 10.
                        {
                            physics
                                .get_mut(player_physics_handle)
                                .unwrap()
                                .apply_force(5. * diff.normalize());
                            info!("PLAYER HIT");
                            enemy.hit_player_cooldown = HIT_PLAYER_COOLDOWN;

                            // Subtract player health
                            for (_, (_, health)) in &mut world.query::<(&Player, &mut Health)>() {
                                let old_health = health.0;
                                health.0 = health.0.saturating_sub(1);
                                info!("PLAYER HEALTH: {:?}", health.0);

                                if old_health > 0 {
                                    play_sound_once(hit_sound);
                                    screen_shake_amount += 4.0;
                                    if health.0 == 0 {
                                        // Player died!
                                        explosions_to_spawn.push(Explosion::new(
                                            player_center,
                                            80.,
                                            0.5,
                                            true,
                                        ))
                                    } else {
                                        // Player was damaged
                                        explosions_to_spawn.push(Explosion::new(
                                            player_center,
                                            30.,
                                            0.6,
                                            true,
                                        ))
                                    }
                                }
                            }
                        }
                    } else {
                        enemy.following_player_sound_cooldown = 0;
                    }
                } else {
                    // Sink when dead
                    p.apply_force(Vec2::Y * 0.02);
                }
            }

            // Accelerate things
            for (_entity, (thing, accelerator)) in &mut world.query::<(&Thing, &Accelerator)>() {
                if let Some(p) = physics.get_mut(thing.physics_handle) {
                    p.apply_force(accelerator.amount)
                }
            }

            let player_physics = {
                let mut query = world
                    .query_one::<(&mut Player, &mut Thing, &Health)>(player_entity)
                    .unwrap();
                let (player, player_thing, health) = query.get().unwrap();
                physics.get_mut(player_thing.physics_handle).unwrap()
            };

            let mut missile_to_spawn = None;
            // Player controls
            {
                let mut query = world
                    .query_one::<(&mut Player, &mut Thing, &Health)>(player_entity)
                    .unwrap();
                let (player, player_thing, health) = query.get().unwrap();

                if health.0 > 0 {
                    // Player controls
                    let player_acceleration = 0.1;

                    let flip = if camera_flipped { -1.0 } else { 1.0 };
                    if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
                        player_physics.apply_force(Vec2::new(-player_acceleration * flip, 0.));
                        player.direction_x = -1.0 * flip;
                    }
                    if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
                        player_physics.apply_force(Vec2::new(player_acceleration * flip, 0.));
                        player.direction_x = 1.0 * flip;
                    }
                    if !above_water && !below_victory_depth {
                        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
                            player_physics.apply_force(Vec2::new(0., -player_acceleration * flip));
                        }
                    }

                    if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
                        player_physics.apply_force(Vec2::new(0., player_acceleration * flip));
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

                    camera_zoom =
                        (max_camera_zoom - min_camera_zoom) * depth_lerp + min_camera_zoom;

                    let camera_buffer = (screen_height / camera_zoom) * 2.0 * 0.1;
                    if player_center.y > camera_focal_y + camera_buffer {
                        camera_focal_y = player_center.y - camera_buffer;
                    }

                    if player_center.y < camera_focal_y - camera_buffer {
                        camera_focal_y = player_center.y + camera_buffer;
                    }

                    let has_missiles = unsafe { UPGRADES_COLLECTED > 0 };
                    // Fire missile
                    if has_missiles && is_key_pressed(KeyCode::Space) {
                        play_sound_once(shoot_sound);
                        let position = player_physics.position
                            + Vec2::new(
                                player.direction_x
                                    * (missile_size.x / 2. + 2.0 + player_half_width),
                                0.,
                            );

                        let acceleration_direction = Vec2::new(player.direction_x, 0.0) * 0.1;
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
                        // set to make missiles not explode against each-other
                        physics_object.is_missile = true;
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
                } else {
                    // Sink when dead
                    player_physics.apply_force(Vec2::Y * 0.02);
                    respawn_timer = respawn_timer.saturating_sub(1);
                }
            }

            // Separated out to avoid borrow checker complaints.

            if let Some((missile_to_spawn, direction)) = missile_to_spawn.take() {
                info!("MISSILE SPAWNING");
                world.spawn((
                    missile_to_spawn,
                    Missile {},
                    Drawable::Missile,
                    Accelerator {
                        amount: Vec2::new(direction * 0.07, 0.0),
                    },
                ));
            }

            let screen_shake = Vec2::new(
                rand::gen_range(-screen_shake_amount, screen_shake_amount),
                rand::gen_range(-screen_shake_amount, screen_shake_amount),
            );

            camera_flip_counter = ((1. / 20.) + camera_flip_counter).min(1.0);

            if camera_flip_counter == 1.0 {
                camera_angle = camera_target_angle;
            }

            let t = camera_flip_counter * camera_flip_counter;

            set_camera(&Camera2D {
                target: vec2(0., camera_focal_y) + screen_shake,
                zoom: Vec2::new(camera_zoom / screen_width, -camera_zoom / screen_height),
                rotation: (camera_target_angle - camera_angle) * t + camera_angle,
                ..Default::default()
            });

            if below_victory_depth && !camera_flipped {
                play_sound_once(hello_sound);
                // Play victory music!
                play_sound(
                    end_music,
                    PlaySoundParams {
                        looped: true,
                        volume: 1.0,
                    },
                );
                camera_target_angle = 180.;
                camera_flip_counter = 0.0;
                camera_flipped = true;
            }

            if above_water && camera_flipped {
                camera_target_angle = 0.0;
                camera_flip_counter = 0.0;
                camera_flipped = false;
            }

            let mut collected = false;
            for (entity, torpedo_upgrade) in &mut world.query::<&TorpedoUpgrade>() {
                if (torpedo_upgrade.position - player_center).length() < TORPEDO_UPGRADE_RADIUS {
                    info!("COLLECT TORPEDO UPGRADE");
                    entities_to_despawn.push(entity);
                    play_sound_once(upgrade_sound);
                    collected = true;
                }
            }
            if collected {
                for (_, player) in &mut world.query::<&mut Player>() {
                    unsafe {
                        UPGRADES_COLLECTED += 1;
                    }
                }
            }

            physics.run();

            // Check for missile impacts
            for (entity, (thing, _missile)) in &mut world.query::<(&Thing, &Missile)>() {
                if let Some(p) = physics.get(thing.physics_handle) {
                    if p.last_collision_impact.length() > 0.5 {
                        // info!("MISSILE IMPACT: {:?}", p.last_collision_impact.length());
                        entities_to_despawn.push(entity);
                        explosions_to_spawn.push(Explosion::new(p.position, 70., 1.0, false));

                        physics.remove(thing.physics_handle)
                    }
                }
            }

            for explosion in explosions_to_spawn.drain(..) {
                screen_shake_amount += 2.0;
                play_sound_once(explosion_sounds[rand::gen_range(0, explosion_sounds.len())]);
                world.spawn((explosion,));
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

            if is_key_pressed(KeyCode::Key4) {
                info!("CHECKPOINT MODE");
                editor_mode = 4;
            }

            if is_key_pressed(KeyCode::Key5) {
                info!("TORPEDO UPGRADE MODE");
                editor_mode = 5;
            }
            let mouse_position_in_world = camera.screen_to_world(mouse_position().into());

            match editor_mode {
                0 => {
                    if is_mouse_button_pressed(MouseButton::Left) {
                        let mut to_despawn = None;
                        for (entity, (thing,)) in world.query::<(&Checkpoint,)>().iter() {
                            let distance = thing.position - mouse_position_in_world;
                            if distance.length() < 20. {
                                to_despawn = Some((entity, Some(thing.index_in_level)));
                            }
                        }

                        for (entity, (thing,)) in world.query::<(&TorpedoUpgrade,)>().iter() {
                            let distance = thing.position - mouse_position_in_world;
                            if distance.length() < 20. {
                                to_despawn = Some((entity, Some(thing.index_in_level)));
                            }
                        }

                        if to_despawn.is_none() {
                            for (entity, (thing,)) in world.query::<(&Thing,)>().iter() {
                                let p = physics.get(thing.physics_handle).unwrap();
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
                        }

                        if let Some((to_despawn, index)) = to_despawn {
                            if let Some(index) = index {
                                // We need to despawn in the physics, the level, and the ECS
                                let _ = world.despawn(to_despawn);
                                level.things.swap_remove(index);

                                // Just reload the entire level, but preserve the player position.
                                player_entity = setup(
                                    &level,
                                    &mut world,
                                    &mut physics,
                                    &mut respawn_timer,
                                    player_center,
                                );
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
                        let p = physics.get_mut(player_thing.physics_handle).unwrap();
                        p.position = mouse_position_in_world;
                        p.last_position = mouse_position_in_world;
                        camera_focal_y = mouse_position_in_world.y;
                    }
                }
                _ => {
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
                                4 => LevelItem {
                                    position: rect.point() + rect.size() / 2.,
                                    half_size: rect.size() / 2.,
                                    thing_type: ThingType::Checkpoint,
                                },
                                5 => LevelItem {
                                    position: rect.point() + rect.size() / 2.,
                                    half_size: rect.size() / 2.,
                                    thing_type: ThingType::TorpedoUpgrade,
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

        if below_victory_depth {
            // Draw sky
            draw_rectangle(
                0. - screen_width / camera_zoom, //-main_area_width / 2.,
                victory_depth,
                screen_width / camera_zoom * 2., //main_area_width,
                30000.,
                Color::new(163. / 255., 217. / 255., 234. / 255., 1.),
            );
        } else if above_water {
            // Draw sky
            draw_rectangle(
                0. - screen_width / camera_zoom, //-main_area_width / 2.,
                -3000.,
                screen_width / camera_zoom * 2., //main_area_width,
                water_level + 3000.,
                Color::new(106. / 255., 183. / 255., 206. / 255., 1.),
            );
        } else {
            // Draw underwater sky
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
                victory_depth - water_level,
                Color::new(27. / 255., 66. / 255., 81. / 255., 1.),
            );

            // Draw other side underwater sky
            draw_rectangle(
                0. - screen_width / camera_zoom, //-main_area_width / 2.,
                victory_depth,
                screen_width / camera_zoom * 2., //main_area_width,
                30000.,
                Color::new(20. / 255., 50. / 255., 70. / 255., 1.),
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
        for (_entity, (thing, drawable, enemy, player, health)) in &mut world.query::<(
            &Thing,
            &Drawable,
            Option<&Enemy>,
            Option<&Player>,
            Option<&Health>,
        )>() {
            let physics_object = physics.get(thing.physics_handle);

            if let Some(physics_object) = physics_object {
                match drawable {
                    Drawable::Player => match physics_object.collider {
                        Collider::Rectangle {
                            half_width,
                            half_height,
                        } => {
                            let health = health.unwrap().0;
                            let color = if health == 0 { GRAY } else { thing.color };
                            draw_rectangle(
                                physics_object.position.x - half_width,
                                physics_object.position.y - half_height,
                                half_width * 2.,
                                half_height * 2.,
                                color,
                            );

                            if player.unwrap().direction_x > 0. {
                                draw_circle(
                                    physics_object.position.x + half_width,
                                    physics_object.position.y,
                                    half_height,
                                    color,
                                );
                            } else {
                                draw_circle(
                                    physics_object.position.x - half_width,
                                    physics_object.position.y,
                                    half_height,
                                    color,
                                );
                            }
                        }
                        _ => unreachable!(),
                    },
                    Drawable::Enemy => match physics_object.collider {
                        Collider::Circle { radius } => {
                            let enemy = enemy.unwrap();
                            let direction = enemy.acceleration_direction.normalize();
                            let rotation = direction.angle_between(-Vec2::X);

                            let texture = if health.unwrap().0 <= 0 {
                                dead_tooth_fish_texture
                            } else {
                                tooth_fish_texture
                            };

                            // Make radius slightly bigger to allow player to have close grazes with enemy
                            let radius = radius * 1.1;
                            draw_texture_ex(
                                texture,
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
                    Drawable::TorpedoUpgrade => {
                        info!("DRAWING TORPEDO UPGRADE");
                        draw_texture_ex(
                            torpedo_upgrade,
                            physics_object.position.x - TORPEDO_UPGRADE_RADIUS,
                            physics_object.position.y - TORPEDO_UPGRADE_RADIUS,
                            WHITE,
                            DrawTextureParams {
                                dest_size: Some(Vec2::new(
                                    TORPEDO_UPGRADE_RADIUS * 2.,
                                    TORPEDO_UPGRADE_RADIUS * 2.,
                                )),
                                ..Default::default()
                            },
                        );
                    }
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
        }

        // Draw and update explosions
        for (entity, explosion) in &mut world.query::<&mut Explosion>() {
            explosion.draw(player_center);
            if explosion.pieces.len() == 0 {
                entities_to_despawn.push(entity);
            }

            // Check if an enemy is within an explosion

            if !explosion.damaged && !explosion.follow_player {
                explosion.damaged = true;
                for (_entity, (thing, _enemy, health)) in
                    &mut world.query::<(&Thing, &mut Enemy, &mut Health)>()
                {
                    let p = physics.get_mut(thing.physics_handle).unwrap();
                    let diff = p.position - explosion.center;

                    let radius = match p.collider {
                        Collider::Circle { radius } => radius,
                        _ => unreachable!(),
                    };

                    let distance = explosion.radius + radius;
                    let distance_squared = distance * distance;

                    let threshold = distance_squared * 1.2;

                    if diff.length_squared() < distance_squared {
                        p.apply_force((distance_squared / threshold) * diff.normalize() * 10.);

                        health.0 = health.0.saturating_sub(1);
                        if health.0 == 0 {
                            // Kill it!
                            p.mass = 0.5;
                            play_sound_once(enemy_death_sound);
                            info!("PLAYING DEATH SOUND");
                        }
                    }
                }
            }
        }

        // Draw water here to draw over entities
        if above_water {
            // Draw water
            draw_rectangle(
                0. - screen_width / camera_zoom, //-main_area_width / 2.,
                water_level,
                screen_width / camera_zoom * 2., //main_area_width,
                victory_depth - water_level,
                Color::new(27. / 255., 66. / 255., 81. / 255., 1.),
            );
        }

        if below_victory_depth {
            // Draw water
            draw_rectangle(
                0. - screen_width / camera_zoom, //-main_area_width / 2.,
                victory_depth - 30000.,
                screen_width / camera_zoom * 2., //main_area_width,
                30000.,
                Color::new(27. / 255., 66. / 255., 81. / 255., 1.),
            );

            draw_texture_ex(
                home_texture,
                0.,
                victory_depth + 400.,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(-400., -400.)),
                    ..Default::default()
                },
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

        // Draw torpedo upgrades
        for (_entity, upgrade) in &mut world.query::<&mut TorpedoUpgrade>() {
            draw_texture_ex(
                torpedo_upgrade,
                upgrade.position.x - TORPEDO_UPGRADE_RADIUS,
                upgrade.position.y - TORPEDO_UPGRADE_RADIUS,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(
                        TORPEDO_UPGRADE_RADIUS * 2.,
                        TORPEDO_UPGRADE_RADIUS * 2.,
                    )),
                    ..Default::default()
                },
            );
        }

        if in_level_editor {
            for (_entity, checkpoint) in &mut world.query::<&mut Checkpoint>() {
                draw_circle(
                    checkpoint.position.x,
                    checkpoint.position.y,
                    10.,
                    Color::new(0., 1., 0., 0.7),
                );
            }
        }

        if respawn_timer != RESPAWN_TIMER {
            let fade = 1.0 - respawn_timer as f32 / RESPAWN_TIMER as f32;
            set_default_camera();
            draw_rectangle(
                0.,
                0.,
                screen_width,
                screen_height,
                Color::new(0., 0., 0., fade),
            );

            // Respawn the player
            if respawn_timer == 0 {
                // Find nearest checkpoint
                let mut closest_checkpoint_position = Vec2::ZERO;
                let mut min_distance = f32::MAX;
                {
                    for (_, (checkpoint,)) in &mut world.query::<(&Checkpoint,)>() {
                        let distance = (checkpoint.position - player_center).length();
                        if distance < min_distance {
                            min_distance = distance;
                            closest_checkpoint_position = checkpoint.position;
                        }
                    }
                };
                player_entity = setup(
                    &level,
                    &mut world,
                    &mut physics,
                    &mut respawn_timer,
                    closest_checkpoint_position,
                );
            }
        }

        for entity in entities_to_despawn.drain(..) {
            let _ = world.despawn(entity);
        }

        next_frame().await
    }
}
