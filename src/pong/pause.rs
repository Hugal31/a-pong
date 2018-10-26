use amethyst::prelude::*;
use amethyst::core::Time;
use amethyst::input::{is_close_requested, is_key_down};
use amethyst::renderer::VirtualKeyCode;

#[derive(Default)]
pub struct Pause {
    saved_time_scale: f32,
}

impl Pause {
    pub fn new() -> Pause {
        Pause::default()
    }
}

impl<'a, 'b> SimpleState<'a, 'b> for Pause {

    fn on_start(&mut self, data: StateData<GameData>) {
        let mut time = data.world.write_resource::<Time>();
        self.saved_time_scale = time.time_scale();
        time.set_time_scale(0.0);
    }

    fn on_stop(&mut self, data: StateData<GameData>) {
        let mut time = data.world.write_resource::<Time>();
        time.set_time_scale(self.saved_time_scale);
    }

    fn handle_event(&mut self, _: StateData<GameData>, event: StateEvent) -> SimpleTrans<'a, 'b> {
        if let StateEvent::Window(wevent) = event {
            if is_close_requested(&wevent) || is_key_down(&wevent, VirtualKeyCode::Escape) {
                return Trans::Quit;
            } else if is_key_down(&wevent, VirtualKeyCode::Space) {
                return Trans::Pop;
            }
        }

        Trans::None
    }
}
