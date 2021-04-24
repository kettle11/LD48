use crate::*;

pub(crate) fn get_level() -> Level {
    Level {
        things: vec![Thing {
            rect: Rect {
                x: 0.,
                y: 0.,
                w: 200.,
                h: 200.,
            },
            color: GREEN,
            friction: 1.0,
            velocity: Vec2::ZERO,
            acceleration: Vec2::ZERO,
        }],
    }
}
