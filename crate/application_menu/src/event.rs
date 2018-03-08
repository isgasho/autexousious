/// Events used to indicate top level transitions for an application.
///
/// # Type Parameters
///
/// * `I`: Type that represents the index of the selected menu item.
#[derive(Clone, Debug, Hash, PartialEq)]
pub enum MenuEvent<I> {
    /// Indicates a menu item was pressed.
    Select(I),
    /// Indicates the menu should be closed.
    ///
    /// TODO: `UiEvent`s currently only include mouse input. Should they also include keyboard /
    /// device input?
    ///
    /// If not, `State`s have to generate this event in [`handle_event(..)`][hdl_evt], instead of
    /// the `UiInputHandlerSystem`s' [`run(..)`][specs_run].
    ///
    /// [hdl_evt]: https://docs.rs/amethyst/0.6.0/amethyst/trait.State.html#method.handle_event
    /// [specs_run]: https://docs.rs/specs/0.10.0/specs/trait.System.html#tymethod.run
    Close,
}