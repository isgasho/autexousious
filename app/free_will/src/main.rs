//! Opens an empty window.

extern crate amethyst;
#[macro_use]
extern crate application;
extern crate application_input;
extern crate application_ui;
extern crate game_mode_menu;
extern crate stdio_view;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;

use std::process;

use amethyst::renderer::{DisplayConfig, DrawFlat, Pipeline, PosNormTex, RenderBundle, Stage};
use amethyst::prelude::*;
use amethyst::ui::{DrawUi, UiBundle};
use application::resource::dir;
use application::resource::find_in;
use application_input::ApplicationInputBundle;
use application_ui::ApplicationUiBundle;
use stdio_view::StdinSystem;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "Free Will")]
struct Opt {
    #[structopt(long = "headless", help = "Run headlessly (no GUI)")]
    headless: bool,
}

fn run(opt: &Opt) -> Result<(), amethyst::Error> {
    let mut app_builder = Application::build(dir::ASSETS, game_mode_menu::State::new())?
        .with_bundle(ApplicationInputBundle::new())?
        .with::<StdinSystem>(StdinSystem::new(), "StdinSystem", &[]);

    if !opt.headless {
        let display_config = DisplayConfig::load(
            find_in(
                dir::RESOURCES,
                "display_config.ron",
                Some(development_base_dirs!()),
            ).unwrap(),
        );

        let pipe = Pipeline::build().with_stage(
            Stage::with_backbuffer()
                .clear_target([0., 0., 0., 1.], 1.)
                .with_pass(DrawFlat::<PosNormTex>::new())
                .with_pass(DrawUi::new()),
        );

        app_builder = app_builder
            .with_bundle(UiBundle::new())?
            .with_bundle(RenderBundle::new(pipe, Some(display_config)))?
            .with_bundle(ApplicationUiBundle::new())?;
    }

    let mut app = app_builder.build().expect("Fatal error");

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
