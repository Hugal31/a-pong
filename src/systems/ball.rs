use amethyst::core::transform::components::Transform;
use amethyst::ecs::{Join, ReadStorage, System, WriteStorage};
use pong::{Ball};

pub struct BallSystem;

impl<'s> System<'s> for BallSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Ball>,
    );

    fn run(&mut self, (mut transforms, ball): Self::SystemData) {
        for (_, transform) in (&ball, &mut transforms).join() {
        }
    }
}
