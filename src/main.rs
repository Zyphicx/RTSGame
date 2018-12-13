mod game;
mod components;
mod systems;

use crate::game::Game;
use crate::components::*;

extern crate amethyst;

use amethyst::core::transform::TransformBundle;
use amethyst::input::InputBundle;
use amethyst::prelude::*;
use amethyst::renderer::{DisplayConfig, DrawFlat2D, Event, Pipeline,
                         RenderBundle, Stage, VirtualKeyCode};
use amethyst::utils::{application_root_dir};

pub struct MainMenu;

impl SimpleState for MainMenu {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
    }
}


fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let config_path = format!("{}/resources/display_config.ron", application_root_dir());
    let config = DisplayConfig::load(&config_path);

    let binding_path = format!("{}/resources/bindings_config.ron", application_root_dir()); 

    let mouse_bindings = Bindings::new()
        .insert_action_binding("lmb"

    let input_bundle = InputBundle::<String, String>::new()
        .with_bindings_from_file(binding_path)?
        .with_bindings()?;

    let pipe = Pipeline::build()
        .with_stage(
            Stage::with_backbuffer()
                .clear_target([1.0, 1.0, 1.0, 1.0], 1.0)
                .with_pass(DrawFlat2D::new()),
        );

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderBundle::new(pipe, Some(config))
                .with_sprite_sheet_processor()
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with(systems::UnitTargetSystem, "unit_target_system", &["input_system"])
        .with(systems::UnitMoveSystem, "unit_move_system", &[])
        .with(systems::TargetSystem {reader: None}, "target_system", &[]);

    let mut game = Application::new("./", Game, game_data)?;

    game.run();

    Ok(())
}