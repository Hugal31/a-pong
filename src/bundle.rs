use amethyst::core::bundle::{Result, SystemBundle};
use amethyst::core::specs::DispatcherBuilder;

use super::systems;

pub struct PongBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for PongBundle {
    fn build(self, dispatcher: &mut DispatcherBuilder<'a, 'b>) -> Result <()> {
        dispatcher.add(systems::PaddleSystem, "paddle_system", &["input_system"]);
        dispatcher.add(systems::GravitySystem, "gravity_system", &[]);
        dispatcher.add(systems::CapSpeedSystem::default(), "cap_speed_system", &["paddle_system", "gravity_system"]);

        Ok(())
    }
}
