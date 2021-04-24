use crate::*;
pub(crate) fn get_level() -> Level {
    Level {
        things: vec![
            Thing {
                rect: Rect {
                    x: -307.0,
                    y: 2667.1777,
                    w: 384.0,
                    h: 506.0,
                },
                ..KINEMATIC_LEVEL_PIECE
            },
            Thing {
                rect: Rect {
                    x: 200.00002,
                    y: 2779.1777,
                    w: 110.000015,
                    h: 132.0,
                },
                ..KINEMATIC_LEVEL_PIECE
            },
            Thing {
                rect: Rect {
                    x: 64.99998,
                    y: 2948.1777,
                    w: 93.00002,
                    h: 79.0,
                },
                ..KINEMATIC_LEVEL_PIECE
            },
            Thing {
                rect: Rect {
                    x: 183.00002,
                    y: 3067.1777,
                    w: 135.99998,
                    h: 126.0,
                },
                ..KINEMATIC_LEVEL_PIECE
            },
            Thing {
                rect: Rect {
                    x: 153.99998,
                    y: 3160.1777,
                    w: 42.000015,
                    h: 26.0,
                },
                ..KINEMATIC_LEVEL_PIECE
            },
            Thing {
                rect: Rect {
                    x: 73.00001,
                    y: 3162.1777,
                    w: 62.999977,
                    h: 24.0,
                },
                ..KINEMATIC_LEVEL_PIECE
            },
        ],
    }
}
