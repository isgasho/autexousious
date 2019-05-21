#![windows_subsystem = "windows"]

use std::process;

use amethyst::{
    assets::HotReloadBundle,
    audio::AudioBundle,
    core::transform::TransformBundle,
    input::InputBundle,
    renderer::{types::DefaultBackend, RenderingSystem},
    ui::UiBundle,
    window::{DisplayConfig, WindowBundle},
    CoreApplication, GameDataBuilder, LogLevelFilter, LoggerConfig,
};
use application::{
    development_base_dirs,
    resource::{
        self,
        dir::{self, assets_dir},
        load_in,
    },
};
use application_event::{AppEvent, AppEventReader};
use application_robot::RobotState;
use character_loading::CharacterLoadingBundle;
use character_selection_stdio::CharacterSelectionStdioBundle;
use collision_audio_loading::CollisionAudioLoadingBundle;
use collision_loading::CollisionLoadingBundle;
use frame_rate::strategy::frame_rate_limit_config;
use game_input::GameInputBundle;
use game_input_model::{ControlBindings, InputConfig};
use game_input_stdio::{ControlInputEventStdinMapper, GameInputStdioBundle};
use game_input_ui::{GameInputUiBundle, InputToControlInputSystem};
use game_mode_selection::{GameModeSelectionStateBuilder, GameModeSelectionStateDelegate};
use game_mode_selection_stdio::GameModeSelectionStdioBundle;
use game_mode_selection_ui::GameModeSelectionUiBundle;
use game_play_stdio::GamePlayStdioBundle;
use loading::{LoadingBundle, LoadingState};
use map_loading::MapLoadingBundle;
use map_selection_stdio::MapSelectionStdioBundle;
use sequence_loading::SequenceLoadingBundle;
use sprite_loading::SpriteLoadingBundle;
use stdio_command_stdio::StdioCommandStdioBundle;
use stdio_input::StdioInputBundle;
use stdio_spi::MapperSystem;
use structopt::StructOpt;
use typename::TypeName;
use ui_audio_loading::UiAudioLoadingBundle;

use crate::render_graph::RenderGraph;

mod render_graph;

#[derive(StructOpt, Debug)]
#[structopt(name = "Will", rename_all = "snake_case")]
struct Opt {
    /// Run headlessly (no GUI).
    #[structopt(long)]
    headless: bool,
    /// Frame rate to run the game at.
    #[structopt(long)]
    frame_rate: Option<u32>,
}

fn run(opt: &Opt) -> Result<(), amethyst::Error> {
    amethyst::start_logger(LoggerConfig {
        level_filter: if cfg!(debug_assertions) {
            LogLevelFilter::Debug
        } else {
            LogLevelFilter::Info
        },
        ..Default::default()
    });

    let assets_dir = assets_dir(Some(development_base_dirs!()))?;

    let game_mode_selection_state =
        GameModeSelectionStateBuilder::new(GameModeSelectionStateDelegate::new())
            .with_bundle(GameModeSelectionUiBundle::new())
            .build();
    let loading_state = LoadingState::<_>::new(game_mode_selection_state);
    let state = RobotState::new(Box::new(loading_state));

    let mut game_data = GameDataBuilder::default();
    if !opt.headless {
        let display_config = load_in::<DisplayConfig, _>(
            dir::RESOURCES,
            "display_config.ron",
            resource::Format::Ron,
            Some(development_base_dirs!()),
        )?;

        let input_config = load_in::<InputConfig, _>(
            dir::RESOURCES,
            "input_config.ron",
            resource::Format::Ron,
            Some(development_base_dirs!()),
        )?;

        // `InputBundle` provides `InputHandler<A, B>`, needed by the `UiBundle` for mouse events.
        // `UiBundle` registers `Loader<FontAsset>`, needed by `ApplicationUiBundle`.
        game_data = game_data
            .with_bundle(WindowBundle::from_config(display_config))?
            .with_bundle(AudioBundle::default())?
            .with_bundle(TransformBundle::new())?
            .with_thread_local(RenderingSystem::<DefaultBackend, _>::new(
                RenderGraph::default(),
            ))
            .with_bundle(
                InputBundle::<ControlBindings>::new().with_bindings((&input_config).into()),
            )?
            .with_bundle(UiBundle::<DefaultBackend, ControlBindings>::new())?
            .with_bundle(HotReloadBundle::default())?
            .with_bundle(SpriteLoadingBundle::new())?
            .with_bundle(SequenceLoadingBundle::new())?
            .with_bundle(LoadingBundle::new(assets_dir.clone()))?
            .with_bundle(GameInputUiBundle::new(input_config))?
            .with_bundle(
                GameInputStdioBundle::new()
                    // Note: Depend on the input handler updated system, so that stdin input takes
                    // priority
                    .with_system_dependencies(&[InputToControlInputSystem::type_name()]),
            )?
            .with_bundle(GameInputBundle::new().with_system_dependencies(&[
                MapperSystem::<ControlInputEventStdinMapper>::type_name(),
                InputToControlInputSystem::type_name(),
            ]))?
            .with_bundle(StdioInputBundle::new())?
            .with_bundle(StdioCommandStdioBundle::new())?
            .with_bundle(CharacterSelectionStdioBundle::new())?
            .with_bundle(GamePlayStdioBundle::new())?
            .with_bundle(GameModeSelectionStdioBundle::new())?
            .with_bundle(MapSelectionStdioBundle::new())?
            .with_bundle(CollisionLoadingBundle::new())?
            .with_bundle(MapLoadingBundle::new())?
            .with_bundle(CharacterLoadingBundle::new())?
            .with_bundle(CollisionAudioLoadingBundle::new(assets_dir.clone()))?
            .with_bundle(UiAudioLoadingBundle::new(assets_dir.clone()))?;
    }

    let mut app = CoreApplication::<_, AppEvent, AppEventReader>::build(assets_dir, state)?
        .with_frame_limit_config(frame_rate_limit_config(opt.frame_rate))
        .build(game_data)?;

    app.run();

    Ok(())
}

fn main() {
    let opt = Opt::from_args();

    if let Err(e) = run(&opt) {
        println!("Failed to execute example: {}", e);
        process::exit(1);
    }
}
