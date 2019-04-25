use std::{error, fmt};

use game_input_model::ControllerId;

/// Errors when mapping events from the `game_input` crates.
#[derive(Clone, Debug, PartialEq)]
pub enum GameInputStdioError {
    /// There is no entity with an `InputControlled` component with the specified controller ID.
    EntityWithControllerIdNotFound {
        /// Controller ID specified by the user.
        controller_id: ControllerId,
        /// Controller IDs of entities with the `InputControlled` component.
        existent_controllers: Vec<ControllerId>,
    },
}

impl fmt::Display for GameInputStdioError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameInputStdioError::EntityWithControllerIdNotFound {
                controller_id,
                existent_controllers,
            } => {
                writeln!(
                    f,
                    "Failed to find entity with an `InputControlled` component with the specified controller ID: `{}`.",
                    controller_id,
                )?;
                writeln!(
                    f,
                    "The following controller IDs are associated with entities:",
                )?;
                writeln!(f)?;
                existent_controllers
                    .iter()
                    .try_for_each(|controller_id| writeln!(f, "* {}", controller_id))?;
                writeln!(f)
            }
        }
    }
}

impl error::Error for GameInputStdioError {}

#[cfg(test)]
mod tests {
    use super::GameInputStdioError;

    #[test]
    fn fmt_display_is_user_friendly() {
        assert_eq!(
            "Failed to find entity with an `InputControlled` component with the specified controller ID: `4`.\n\
             The following controller IDs are associated with entities:\n\
             \n\
             * 0\n\
             * 1\n\
             \n\
            ",
            format!("{}", GameInputStdioError::EntityWithControllerIdNotFound { controller_id: 4, existent_controllers: vec![0, 1] })
        );
    }
}
