use amethyst::{
    core::EventReader,
    derive::EventReader,
    ecs::{Read, World},
    shred::SystemData,
    shrev::{EventChannel, ReaderId},
    winit::event::Event,
};
use asset_selection_model::play::AssetSelectionEvent;
use control_settings_model::ControlSettingsEvent;
use derive_more::From;
use game_input_model::play::ControlInputEvent;
use game_mode_selection_model::GameModeSelectionEvent;
use game_play_model::GamePlayEvent;
use network_mode_selection_model::NetworkModeSelectionEvent;
use session_host_model::SessionHostEvent;
use session_join_model::SessionJoinEvent;
use session_lobby_model::SessionLobbyEvent;
use stdio_command_model::StdioCommandEvent;
use strum_macros::{Display, EnumDiscriminants, EnumIter, EnumString};

/// Type encompassing all state event types.
#[derive(Clone, Debug, Display, EnumDiscriminants, EventReader, From, PartialEq)]
#[strum_discriminants(
    name(AppEventVariant),
    derive(Display, EnumIter, EnumString),
    strum(serialize_all = "snake_case")
)]
#[reader(AppEventReader)]
pub enum AppEvent {
    /// `asset_selection` events.
    AssetSelection(AssetSelectionEvent),
    /// `control_input` events.
    ///
    /// Note: This is defined in the `game_input*` crates.
    ControlInput(ControlInputEvent),
    /// `control_settings` events.
    ControlSettings(ControlSettingsEvent),
    /// `game_mode_selection` events.
    GameModeSelection(GameModeSelectionEvent),
    /// `game_play` events.
    GamePlay(GamePlayEvent),
    /// `network_mode_selection` events.
    NetworkModeSelection(NetworkModeSelectionEvent),
    /// `session_host` events.
    SessionHost(SessionHostEvent),
    /// `session_join` events.
    SessionJoin(SessionJoinEvent),
    /// `session_lobby` events.
    SessionLobby(SessionLobbyEvent),
    /// `stdio_command` events.
    StdioCommand(StdioCommandEvent),
    /// Events sent by the winit window.
    Window(Event<'static, ()>),
}
