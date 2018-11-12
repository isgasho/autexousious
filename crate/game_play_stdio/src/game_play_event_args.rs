/// Parameters to the mapper.
///
/// # Examples
///
/// * `game_play return`
/// * `game_play restart`
/// * `game_play pause`
/// * `game_play resume`
/// * `game_play end`
/// * `game_play end_stats`
#[derive(Clone, Copy, Debug, PartialEq, StructOpt)]
#[structopt(rename_all = "snake_case")]
pub enum GamePlayEventArgs {
    /// Returns to the menu.
    Return,
    /// Restarts the round.
    Restart,
    /// Pauses the round.
    Pause,
    /// Resumes the round.
    Resume,
    /// Signals the end of the round.
    End,
    /// Signals to go to the round statistics.
    EndStats,
}
