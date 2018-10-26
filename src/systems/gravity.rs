use amethyst::core::cgmath::Vector2;
use amethyst::ecs::{Join, Read, System, WriteStorage};

use amethyst_rhusics::rhusics_core::NextFrame;
use amethyst_rhusics::rhusics_ecs::DeltaTime;

use pong::Velocity;

#[derive(Default)]
pub struct Gravity(pub f32);

pub struct GravitySystem;

impl<'s> System<'s> for GravitySystem {
    type SystemData = (
        Read<'s, Gravity>,
        Read<'s, DeltaTime<f32>>,
        WriteStorage<'s, NextFrame<Velocity>>,
    );

    fn run(&mut self, (gravity, time, mut velocities): Self::SystemData) {
        for velocity in (&mut velocities).join() {
            let linear = velocity.value.linear().clone();
            velocity.value.set_linear(Vector2::new(linear.x, linear.y + gravity.0 * time.delta_seconds));
        }
    }
}
