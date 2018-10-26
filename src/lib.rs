extern crate amethyst;
extern crate amethyst_rhusics;
#[macro_use]
extern crate log;

mod bundle;
mod pong;
mod systems;

pub use bundle::PongBundle;
pub use pong::Pong as StartState;
