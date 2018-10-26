use std::f32::consts;

use amethyst::core::cgmath::{Rotation, Vector2};
use amethyst::core::timing::Time;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::InputHandler;
use amethyst_rhusics::rhusics_core::{NextFrame, Pose};

use pong::{BodyPose, Paddle, Side, Velocity};

pub struct PaddleSystem;

impl PaddleSystem {
    const LINEAR_VELOCITY: f32 = 100.0;
    const ANGULAR_VELOCITY: f32 = consts::PI * 2.0;
}

// TODO See if we can put this before the physical engine computation, to avoid using "NextFrame"
impl<'s> System<'s> for PaddleSystem {
    type SystemData = (
        WriteStorage<'s, NextFrame<Velocity>>,
        ReadStorage<'s, BodyPose>,
        ReadStorage<'s, Paddle>,
        Read<'s, InputHandler<String, String>>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut velocities, poses, paddles, input, time): Self::SystemData) {
        for (paddle, pose, velocity) in (&paddles, &poses, &mut velocities).join() {
            let vertical_movement = match paddle.side {
                Side::Left => input.axis_value("vertical_left_paddle"),
                Side::Right => input.axis_value("vertical_right_paddle"),
            };

            if let Some(mv_amount) = vertical_movement {
                let scaled_amount = Self::LINEAR_VELOCITY * time.delta_seconds() * mv_amount as f32;
                let linear = pose.rotation().rotate_vector(Vector2::<f32>::new(0.0, scaled_amount)) + velocity.value.linear();
                velocity.value.set_linear(linear);
            }

            let angular_movement = match paddle.side {
                Side::Left => input.axis_value("rotation_left_paddle"),
                Side::Right => input.axis_value("rotation_right_paddle"),
            };

            if let Some(mv_amount) = angular_movement {
                let scaled_amount = Self::ANGULAR_VELOCITY * time.delta_seconds() * mv_amount as f32;
                let angular = scaled_amount + velocity.value.angular();
                velocity.value.set_angular(angular);
            }
        }
    }
}
