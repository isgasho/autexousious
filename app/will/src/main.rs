#![windows_subsystem = "windows"]

//! Opens an empty window.

use std::{process, time::Duration};

use amethyst::{
    animation::AnimationBundle,
    core::{frame_limiter::FrameRateLimitStrategy, transform::TransformBundle},
    input::InputBundle,
    prelude::*,
    renderer::{
        ColorMask, DepthMode, DisplayConfig, DrawFlat2D, Pipeline, RenderBundle, SpriteRender,
        Stage, ALPHA,
    },
    ui::{DrawUi, UiBundle},
    LogLevelFilter, LoggerConfig,
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
use application_state::{HookFn, HookableFn};
use character_model::config::CharacterSequenceId;
use character_selection_stdio::CharacterSelectionStdioBundle;
use collision_loading::CollisionLoadingBundle;
use collision_model::animation::{BodyFrameActiveHandle, InteractionFrameActiveHandle};
use game_input::GameInputBundle;
use game_input_model::{InputConfig, PlayerActionControl, PlayerAxisControl};
use game_input_stdio::{ControlInputEventStdinMapper, GameInputStdioBundle};
use game_input_ui::{GameInputUiBundle, InputToControlInputSystem};
use game_mode_selection::{GameModeSelectionStateBuilder, GameModeSelectionStateDelegate};
use game_mode_selection_stdio::GameModeSelectionStdioBundle;
use game_mode_selection_ui::{GameModeSelectionUiBuildFn, GameModeSelectionUiBundle};
use game_play_stdio::GamePlayStdioBundle;
use loading::LoadingState;
use log::info;
use map_loading::MapLoadingBundle;
use map_selection_stdio::MapSelectionStdioBundle;
use object_loading::ObjectLoadingBundle;
use stdio_spi::MapperSystem;
use stdio_view::StdioViewBundle;
use structopt::StructOpt;
use typename::TypeName;

#[derive(StructOpt, Debug)]
#[structopt(name = "Will")]
struct Opt {
    #[structopt(long = "headless", help = "Run headlessly (no GUI)")]
    headless: bool,
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
            .with_hook_fn(
                HookableFn::OnStart,
                HookFn(*GameModeSelectionUiBuildFn::new()),
            )
            .with_hook_fn(
                HookableFn::OnResume,
                HookFn(*GameModeSelectionUiBuildFn::new()),
            )
            .build();
    let loading_state = LoadingState::<_>::new(assets_dir.clone(), game_mode_selection_state);
    let state = RobotState::new(Box::new(loading_state));

    let mut game_data = GameDataBuilder::default();
    if !opt.headless {
        let display_config = load_in::<DisplayConfig, _>(
            dir::RESOURCES,
            "display_config.ron",
            resource::Format::Ron,
            Some(development_base_dirs!()),
        )?;

        let pipe = Pipeline::build().with_stage(
            Stage::with_backbuffer()
                .clear_target([0., 0., 0., 1.], 0.)
                .with_pass(DrawFlat2D::new().with_transparency(
                    ColorMask::all(),
                    ALPHA,
                    Some(DepthMode::LessEqualWrite),
                ))
                .with_pass(DrawUi::new()),
        );

        let input_config = load_in::<InputConfig, _>(
            dir::RESOURCES,
            "input_config.ron",
            resource::Format::Ron,
            Some(development_base_dirs!()),
        )?;

        // `InputBundle` provides `InputHandler<A, B>`, needed by the `UiBundle` for mouse events.
        // `UiBundle` registers `Loader<FontAsset>`, needed by `ApplicationUiBundle`.
        game_data = game_data
            // === Animation bundles === //
            //
            // Shorthand:
            //
            // * `acs`: animation control system
            // * `sis`: sampler_interpolation_system
            //
            // Object/Character animations
            .with_bundle(AnimationBundle::<CharacterSequenceId, SpriteRender>::new(
                "character_sprite_acs",
                "character_sprite_sis",
            ))?
            .with_bundle(
                AnimationBundle::<CharacterSequenceId, BodyFrameActiveHandle>::new(
                    "character_body_frame_acs",
                    "character_body_frame_sis",
                ),
            )?
            .with_bundle(AnimationBundle::<
                CharacterSequenceId,
                InteractionFrameActiveHandle,
            >::new(
                "character_interaction_frame_acs",
                "character_interaction_frame_sis",
            ))?
            // Used for map layer animations.
            .with_bundle(AnimationBundle::<u32, SpriteRender>::new(
                "map_layer_sprite_acs",
                "map_layer_sprite_sis",
            ))?
            // Handles transformations of textures
            .with_bundle(TransformBundle::new())?
            .with_bundle(
                RenderBundle::new(pipe, Some(display_config))
                    .with_sprite_visibility_sorting(&["transform_system"])
                    .with_sprite_sheet_processor(),
            )?
            .with_bundle(
                InputBundle::<PlayerAxisControl, PlayerActionControl>::new()
                    .with_bindings((&input_config).into()),
            )?
            .with_bundle(UiBundle::<PlayerAxisControl, PlayerActionControl>::new())?
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
            .with_bundle(StdioViewBundle::new())?
            .with_bundle(CharacterSelectionStdioBundle::new())?
            .with_bundle(GamePlayStdioBundle::new())?
            .with_bundle(GameModeSelectionStdioBundle::new())?
            .with_bundle(MapSelectionStdioBundle::new())?
            .with_bundle(CollisionLoadingBundle::new())?
            .with_bundle(MapLoadingBundle::new())?
            .with_bundle(ObjectLoadingBundle::new())?;
    }

    info!("Building application.");
    let mut app = CoreApplication::<_, AppEvent, AppEventReader>::build(assets_dir, state)?
        .with_frame_limit(
            FrameRateLimitStrategy::SleepAndYield(Duration::from_micros(1000)),
            60,
        )
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
