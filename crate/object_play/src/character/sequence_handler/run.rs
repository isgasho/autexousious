use object_model::{
    config::object::CharacterSequenceId,
    entity::{CharacterInput, CharacterStatus, CharacterStatusUpdate, ObjectStatusUpdate},
};

use character::sequence_handler::SequenceHandler;

/// Hold forward to run, release to stop running.
#[derive(Debug)]
pub(crate) struct Run;

impl SequenceHandler for Run {
    fn update(
        input: &CharacterInput,
        character_status: &CharacterStatus,
        sequence_ended: bool,
    ) -> CharacterStatusUpdate {
        // Should always be `RunCounter::Unused`
        let run_counter = None;
        // Don't change facing direction
        let mirrored = None;

        let object_status = &character_status.object_status;
        let sequence_id = if (input.x_axis_value < 0. && object_status.mirrored)
            || (input.x_axis_value > 0. && !object_status.mirrored)
        {
            if sequence_ended {
                Some(CharacterSequenceId::Run)
            } else {
                None
            }
        } else {
            Some(CharacterSequenceId::StopRun)
        };

        CharacterStatusUpdate::new(run_counter, ObjectStatusUpdate::new(sequence_id, mirrored))
    }
}

#[cfg(test)]
mod test {
    use object_model::{
        config::object::CharacterSequenceId,
        entity::{
            CharacterInput, CharacterStatus, CharacterStatusUpdate, ObjectStatus,
            ObjectStatusUpdate, RunCounter,
        },
    };

    use super::Run;
    use character::sequence_handler::SequenceHandler;

    #[test]
    fn reverts_to_stop_run_when_no_input() {
        let input = CharacterInput::new(0., 0., false, false, false, false);

        assert_eq!(
            CharacterStatusUpdate::new(
                None,
                ObjectStatusUpdate::new(Some(CharacterSequenceId::StopRun), None)
            ),
            Run::update(
                &input,
                &CharacterStatus::new(
                    RunCounter::Unused,
                    ObjectStatus::new(CharacterSequenceId::Run, false)
                ),
                false
            )
        );
    }

    #[test]
    fn keeps_running_when_x_axis_positive_and_non_mirrored() {
        let input = CharacterInput::new(1., 0., false, false, false, false);

        assert_eq!(
            CharacterStatusUpdate::new(None, ObjectStatusUpdate::new(None, None)),
            Run::update(
                &input,
                &CharacterStatus::new(
                    RunCounter::Unused,
                    ObjectStatus::new(CharacterSequenceId::Run, false)
                ),
                false
            )
        );
    }

    #[test]
    fn keeps_running_when_x_axis_negative_and_mirrored() {
        let input = CharacterInput::new(-1., 0., false, false, false, false);

        assert_eq!(
            CharacterStatusUpdate::new(None, ObjectStatusUpdate::new(None, None)),
            Run::update(
                &input,
                &CharacterStatus::new(
                    RunCounter::Unused,
                    ObjectStatus::new(CharacterSequenceId::Run, true)
                ),
                false
            )
        );
    }

    #[test]
    fn restarts_run_when_sequence_ended() {
        vec![(1., false), (-1., true)]
            .into_iter()
            .for_each(|(x_input, mirrored)| {
                let input = CharacterInput::new(x_input, 0., false, false, false, false);

                assert_eq!(
                    CharacterStatusUpdate::new(
                        None,
                        ObjectStatusUpdate::new(Some(CharacterSequenceId::Run), None)
                    ),
                    Run::update(
                        &input,
                        &CharacterStatus::new(
                            RunCounter::Unused,
                            ObjectStatus::new(CharacterSequenceId::Run, mirrored)
                        ),
                        true
                    )
                );
            });
    }

    #[test]
    fn reverts_to_stop_run_when_x_axis_negative_and_non_mirrored() {
        let input = CharacterInput::new(-1., 0., false, false, false, false);

        assert_eq!(
            CharacterStatusUpdate::new(
                None,
                ObjectStatusUpdate::new(Some(CharacterSequenceId::StopRun), None)
            ),
            Run::update(
                &input,
                &CharacterStatus::new(
                    RunCounter::Unused,
                    ObjectStatus::new(CharacterSequenceId::Run, false)
                ),
                false
            )
        );
    }

    #[test]
    fn reverts_to_stop_run_when_x_axis_positive_and_mirrored() {
        let input = CharacterInput::new(1., 0., false, false, false, false);

        assert_eq!(
            CharacterStatusUpdate::new(
                None,
                ObjectStatusUpdate::new(Some(CharacterSequenceId::StopRun), None)
            ),
            Run::update(
                &input,
                &CharacterStatus::new(
                    RunCounter::Unused,
                    ObjectStatus::new(CharacterSequenceId::Run, true)
                ),
                false
            )
        );
    }

    #[test]
    fn keeps_running_when_x_axis_positive_z_axis_non_zero_and_non_mirrored() {
        let input = CharacterInput::new(1., 1., false, false, false, false);

        assert_eq!(
            CharacterStatusUpdate::new(None, ObjectStatusUpdate::new(None, None)),
            Run::update(
                &input,
                &CharacterStatus::new(
                    RunCounter::Unused,
                    ObjectStatus::new(CharacterSequenceId::Run, false)
                ),
                false
            )
        );

        let input = CharacterInput::new(1., -1., false, false, false, false);

        assert_eq!(
            CharacterStatusUpdate::new(None, ObjectStatusUpdate::new(None, None)),
            Run::update(
                &input,
                &CharacterStatus::new(
                    RunCounter::Unused,
                    ObjectStatus::new(CharacterSequenceId::Run, false)
                ),
                false
            )
        );
    }
}
