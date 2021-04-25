use crate::*;
pub(crate) fn get_level() -> Level {
    Level {
        things: vec![
            LevelItem {
                position: Vec2::new(-23.333359, 854.6667),
                half_size: Vec2::new(173.3333, 225.00006),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(161.93497, 1183.217),
                half_size: Vec2::new(80.96745, 74.8949),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(-96.66668, 1336.3335),
                half_size: Vec2::new(93.33338, 109.99994),
                thing_type: ThingType::Enemy,
            },
        ],
    }
}
