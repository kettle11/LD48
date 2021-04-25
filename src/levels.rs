use crate::*;
pub(crate) fn get_level() -> Level {
    Level {
        things: vec![
            LevelItem {
                position: Vec2::new(-5240.0, 4673.0),
                half_size: Vec2::new(4980.0, 4220.0),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(4460.0, 4683.0),
                half_size: Vec2::new(4139.9995, 4149.9995),
                thing_type: ThingType::Rock,
            },
        ],
    }
}
