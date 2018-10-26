use amethyst::ecs::{Join, System, WriteStorage};
use amethyst_rhusics::rhusics_core::NextFrame;

use pong::{Velocity};

/// Cap the velocity to a maximum speed
pub struct CapSpeedSystem {
    max_norm: f32,
    max_squared_norm: f32,
    max_angular_speed: f32,
}

impl CapSpeedSystem {
    pub fn new(max_speed: f32, max_angular_speed: f32) -> CapSpeedSystem {
        CapSpeedSystem {
            max_norm: max_speed,
            max_squared_norm: max_speed * max_speed,
            max_angular_speed,
        }
    }
}

impl Default for CapSpeedSystem {
    fn default() -> Self {
        Self::new(50.0, ::std::f32::consts::PI * 1.0)
    }
}

impl<'s> System<'s> for CapSpeedSystem {
    type SystemData = WriteStorage<'s, NextFrame<Velocity>>;

    fn run(&mut self, mut velocities: Self::SystemData) {
        for velocity in (&mut velocities).join() {
            // Cap linear velocity
            let linear_velocity = velocity.value.linear().clone();
            let squared_norm = linear_velocity.x * linear_velocity.x
                + linear_velocity.y * linear_velocity.y;
            if squared_norm > self.max_squared_norm {
                let ratio = self.max_norm / squared_norm.sqrt();
                velocity.value.set_linear(linear_velocity.map(|n| n * ratio));
            }

            // Cap angular velocity
            if *velocity.value.angular() > self.max_angular_speed {
                velocity.value.set_angular(self.max_angular_speed)
            } else if *velocity.value.angular() < -self.max_angular_speed {
                velocity.value.set_angular(-self.max_angular_speed)
            }
        }
    }
}
