use crate::*;
pub enum Collider {
    Rectangle { half_width: f32, half_height: f32 },
    Circle { radius: f32 },
}
pub struct PhysicsObject {
    pub mass: f32,
    pub position: Vec2,
    pub last_position: Vec2,
    pub collider: Collider,
    pub gravity_multiplier: f32,
    pub friction: f32,
}
impl PhysicsObject {
    pub fn new(
        mass: f32,
        position: Vec2,
        collider: Collider,
        gravity_multiplier: f32,
        friction: f32,
    ) -> Self {
        Self {
            mass,
            position,
            last_position: position,
            collider,
            gravity_multiplier,
            friction,
        }
    }

    pub fn apply_force(&mut self, amount: Vec2) {
        self.last_position -= amount;
    }

    pub fn velocity(&self) -> Vec2 {
        self.position - self.last_position
    }
}

pub struct Physics {
    pub objects: Vec<PhysicsObject>,
    pub gravity: f32,
    pub friction: f32,
    /// Only apply gravity above water-level
    pub water_level: f32,
}

#[derive(Copy, Clone, Debug)]
pub struct PhysicsHandle(usize);

impl PhysicsHandle {
    pub const fn empty() -> Self {
        Self(0)
    }
}

impl Physics {
    pub fn new(water_level: f32) -> Self {
        Self {
            objects: Vec::new(),
            gravity: 2.0,
            friction: 0.98,
            water_level,
        }
    }

    pub fn get(&self, physics_handle: PhysicsHandle) -> &PhysicsObject {
        &self.objects[physics_handle.0]
    }

    pub fn get_mut(&mut self, physics_handle: PhysicsHandle) -> &mut PhysicsObject {
        &mut self.objects[physics_handle.0]
    }

    pub fn push(&mut self, physics_object: PhysicsObject) -> PhysicsHandle {
        self.objects.push(physics_object);
        PhysicsHandle(self.objects.len() - 1)
    }

    pub fn apply_force(&mut self, handle: PhysicsHandle, amount: Vec2) {
        self.objects[handle.0].last_position -= amount;
    }

    pub fn run(&mut self) {
        for object in &mut self.objects {
            let temp = object.position;
            object.position =
                object.position + (object.position - object.last_position) * object.friction;
            object.last_position = temp;

            // Only apply gravity above water level
            if object.position.y < self.water_level {
                if (object.position.y - self.water_level).abs() < 1.0 {
                    // Apply a little buoyancy
                    object.position.y -= 0.07 * object.gravity_multiplier;
                } else {
                    object.position.y += self.gravity * object.gravity_multiplier;
                }
            } else {
            }
        }

        let len = self.objects.len();
        for i in 0..len {
            for j in i + 1..len {
                fn circle_collision(
                    p0: &mut Vec2,
                    p1: &mut Vec2,
                    r0: f32,
                    r1: f32,
                    mass_ratio: f32,
                ) {
                    let diff = *p1 - *p0;
                    let radius_sum = r0 + r1;
                    if diff.length() < radius_sum {
                        let offset = diff - diff.normalize() * radius_sum;
                        *p0 += offset * mass_ratio;
                        *p1 -= offset * (1.0 - mass_ratio);
                    }
                }

                fn rectangle_rectangle(
                    p0: &mut Vec2,
                    p1: &mut Vec2,
                    h0: Vec2,
                    h1: Vec2,
                    mass_ratio: f32,
                ) {
                    let diff = *p1 - *p0;
                    let half_sum = h0 + h1;

                    let penetration = half_sum - Vec2::new(diff.x.abs(), diff.y.abs());

                    if penetration.x > 0. && penetration.y > 0. {
                        let offset_horizontal = Vec2::new(-penetration.x * diff.x.signum(), 0.);
                        let offset_vertical = Vec2::new(0., -penetration.y * diff.y.signum());

                        let diff = penetration.x - penetration.y;

                        let offset = if diff < 0. {
                            // Push apart along y
                            offset_horizontal
                        } else {
                            offset_vertical
                        };

                        *p0 += offset * mass_ratio;
                        *p1 -= offset * (1.0 - mass_ratio);
                    }
                }

                fn circle_rectangle(
                    circle_position: &mut Vec2,
                    rect_position: &mut Vec2,
                    radius: f32,
                    half_width: f32,
                    half_height: f32,
                    mass_ratio: f32,
                ) {
                    let diff = *circle_position - *rect_position;
                    let circle_distance = (diff).abs();
                    let halfs = Vec2::new(half_width + radius, half_height + radius);
                    let offsets = (circle_distance - halfs) * diff.signum();

                    if !(circle_distance.x > halfs.x) && !(circle_distance.y > halfs.y) {
                        if circle_distance.x <= half_width {
                            circle_position.y -= offsets.y * mass_ratio;
                            rect_position.y += offsets.y * (1.0 - mass_ratio);
                        } else if circle_distance.y <= half_height {
                            circle_position.x -= offsets.x * mass_ratio;
                            rect_position.x += offsets.x * (1.0 - mass_ratio);
                        } else {
                            let corner_distance = (circle_distance.x - half_width).powf(2.)
                                + (circle_distance.y - half_height).powf(2.);
                            let radius_squared = radius.powf(2.);
                            if corner_distance <= radius_squared {
                                let offset = (corner_distance.sqrt() - radius_squared.sqrt()).abs();
                                let offsets = offset * -diff.normalize();

                                *circle_position -= offsets * mass_ratio;
                                *rect_position += offsets * (1.0 - mass_ratio);
                            }
                        }
                    }
                }

                let (object0, object1) = index_twice(&mut self.objects, i, j);
                if object0.mass == f32::INFINITY && object1.mass == f32::INFINITY {
                    continue;
                }
                let mass_total = object1.mass + object0.mass;
                let mut mass_ratio = object1.mass / mass_total;

                if object1.mass == f32::INFINITY {
                    mass_ratio = 1.0;
                }

                match object0.collider {
                    Collider::Circle { radius } => {
                        let r0 = radius;
                        match object1.collider {
                            Collider::Circle { radius } => {
                                let r1 = radius;
                                circle_collision(
                                    &mut object0.position,
                                    &mut object1.position,
                                    r0,
                                    r1,
                                    mass_ratio,
                                )
                            }
                            Collider::Rectangle {
                                half_width,
                                half_height,
                            } => circle_rectangle(
                                &mut object0.position,
                                &mut object1.position,
                                r0,
                                half_width,
                                half_height,
                                mass_ratio,
                            ),
                        }
                    }
                    Collider::Rectangle {
                        half_width,
                        half_height,
                    } => {
                        let h0 = Vec2::new(half_width, half_height);
                        match object1.collider {
                            Collider::Circle { radius } => {
                                let mut mass_ratio = object0.mass / mass_total;
                                if object0.mass == f32::INFINITY {
                                    mass_ratio = 1.0;
                                }

                                circle_rectangle(
                                    &mut object1.position,
                                    &mut object0.position,
                                    radius,
                                    half_width,
                                    half_height,
                                    mass_ratio,
                                )
                            }
                            Collider::Rectangle {
                                half_width,
                                half_height,
                            } => rectangle_rectangle(
                                &mut object0.position,
                                &mut object1.position,
                                h0,
                                Vec2::new(half_width, half_height),
                                mass_ratio,
                            ),
                        }
                    }
                }
            }
        }
    }
}

/// A helper to get two mutable borrows from the same slice.
fn index_twice<T>(slice: &mut [T], first: usize, second: usize) -> (&mut T, &mut T) {
    if first < second {
        let (a, b) = slice.split_at_mut(second);
        (&mut a[first], &mut b[0])
    } else {
        let (a, b) = slice.split_at_mut(first);
        (&mut b[0], &mut a[second])
    }
}
