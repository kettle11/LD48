use crate::*;
pub(crate) fn get_level() -> Level {
    Level {
        things: vec![
            Thing {
                rect: Rect {
                    x: 0.0,
                    y: 0.0,
                    w: 200.0,
                    h: 200.0,
                },
                velocity: Vec2::new(0.0, 0.0),
                acceleration: Vec2::new(0.0, 0.0),
                color: Color {
                    r: 0.0,
                    g: 0.89,
                    b: 0.19,
                    a: 1.0,
                },
                friction: 1.0,
            },
            Thing {
                rect: Rect {
                    x: 151.5,
                    y: 2790.2017,
                    w: 145.0,
                    h: 315.0,
                },
                velocity: Vec2::new(0.0, 0.0),
                acceleration: Vec2::new(0.0, 0.0),
                color: Color {
                    r: 0.0,
                    g: 0.47,
                    b: 0.95,
                    a: 1.0,
                },
                friction: 0.0,
            },
        ],
    }
}
