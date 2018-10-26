extern crate amethyst;
extern crate amethyst_rhusics;
extern crate fern;
extern crate log;

extern crate a_pong;

mod logger;

use amethyst::prelude::*;
use amethyst::input::InputBundle;
use amethyst::core::transform::TransformBundle;
use amethyst::renderer::{ALPHA, ColorMask,
                         DisplayConfig,
                         DrawSprite, Pipeline,
                         RenderBundle, Stage};
use amethyst_rhusics::DefaultPhysicsBundle2;

fn main() -> Result<(), amethyst::Error> {
    logger::start_logger(Default::default());

    let display_path = format!(
        "{}/resources/display_config.ron",
        env!("CARGO_MANIFEST_DIR")
    );
    let display_config = DisplayConfig::load(&display_path);

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
            .with_pass(DrawSprite::new()
                       .with_transparency(ColorMask::all(), ALPHA, None)),
    );

    let binding_path = format!(
        "{}/resources/bindings_config.ron",
        env!("CARGO_MANIFEST_DIR")
    );
    let input_bundle = InputBundle::<String, String>::new().with_bindings_from_file(binding_path)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(RenderBundle::new(pipe, Some(display_config))
                     .with_sprite_sheet_processor())?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with_bundle(DefaultPhysicsBundle2::<()>::new())?
        .with_bundle(a_pong::PongBundle)?;

    let mut game = Application::build("./resources", a_pong::StartState)?
        .build(game_data)?;
    game.run();
    Ok(())
}
