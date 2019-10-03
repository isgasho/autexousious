#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Contains tests for all workspace crates.

#[cfg(test)]
#[macro_use]
extern crate hamcrest;
#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

#[cfg(test)]
mod application;
#[cfg(test)]
mod application_menu;
#[cfg(test)]
mod application_robot;
#[cfg(test)]
mod application_state;
#[cfg(test)]
mod application_test_support;
#[cfg(test)]
mod application_ui;
#[cfg(test)]
mod asset_gfx_gen;
#[cfg(test)]
mod asset_loading;
#[cfg(test)]
mod asset_model;
#[cfg(test)]
mod audio_loading;
#[cfg(test)]
mod audio_play;
#[cfg(test)]
mod camera_play;
#[cfg(test)]
mod character_loading;
#[cfg(test)]
mod character_model;
#[cfg(test)]
mod character_play;
#[cfg(test)]
mod character_prefab;
#[cfg(test)]
mod character_selection;
#[cfg(test)]
mod character_selection_stdio;
#[cfg(test)]
mod character_selection_ui;
#[cfg(test)]
mod charge_play;
#[cfg(test)]
mod chase_play;
#[cfg(test)]
mod collision_audio_loading;
#[cfg(test)]
mod collision_audio_play;
#[cfg(test)]
mod collision_loading;
#[cfg(test)]
mod collision_model;
#[cfg(test)]
mod collision_play;
#[cfg(test)]
mod debug_util_amethyst;
#[cfg(test)]
mod energy_loading;
#[cfg(test)]
mod energy_model;
#[cfg(test)]
mod energy_prefab;
#[cfg(test)]
mod game_input;
#[cfg(test)]
mod game_input_model;
#[cfg(test)]
mod game_input_stdio;
#[cfg(test)]
mod game_input_ui;
