use character_model::config::CharacterSequenceId;

use crate::{
    sequence_handler::{
        common::{
            grounding::AirborneCheck,
            input::{WalkNoMovementCheck, WalkXMovementCheck, WalkZMovementCheck},
            status::AliveCheck,
        },
        CharacterSequenceHandler,
    },
    CharacterSequenceUpdateComponents,
};

#[derive(Debug)]
pub(crate) struct Walk;

impl CharacterSequenceHandler for Walk {
    fn update(components: CharacterSequenceUpdateComponents<'_>) -> Option<CharacterSequenceId> {
        [
            AliveCheck::update,
            AirborneCheck::update,
            WalkNoMovementCheck::update,
            WalkXMovementCheck::update,
            WalkZMovementCheck::update,
        ]
        .iter()
        .fold(None, |status_update, fn_update| {
            status_update.or_else(|| fn_update(components))
        })
    }
}

#[cfg(test)]
mod test {
    use character_model::{config::CharacterSequenceId, play::RunCounter};
    use game_input::ControllerInput;
    use kinematic_model::config::{Position, Velocity};
    use object_model::play::{Grounding, HealthPoints, Mirrored};
    use sequence_model::play::SequenceStatus;

    use super::Walk;
    use crate::{sequence_handler::CharacterSequenceHandler, CharacterSequenceUpdateComponents};

    #[test]
    fn reverts_to_stand_when_no_input() {
        let input = ControllerInput::new(0., 0., false, false, false, false);

        assert_eq!(
            Some(CharacterSequenceId::Stand),
            Walk::update(CharacterSequenceUpdateComponents::new(
                &input,
                HealthPoints::default(),
                CharacterSequenceId::Walk,
                SequenceStatus::default(),
                &Position::default(),
                &Velocity::default(),
                Mirrored::default(),
                Grounding::default(),
                RunCounter::Increase(10)
            ))
        );
    }

    #[test]
    fn reverts_to_stand_with_run_counter_unused_when_no_input_and_run_counter_exceeded() {
        let input = ControllerInput::new(0., 0., false, false, false, false);

        assert_eq!(
            Some(CharacterSequenceId::Stand),
            Walk::update(CharacterSequenceUpdateComponents::new(
                &input,
                HealthPoints::default(),
                CharacterSequenceId::Walk,
                SequenceStatus::default(),
                &Position::default(),
                &Velocity::default(),
                Mirrored::default(),
                Grounding::default(),
                RunCounter::Exceeded
            ))
        );
    }

    #[test]
    fn walk_when_x_axis_positive_mirror() {
        let input = ControllerInput::new(1., 0., false, false, false, false);

        assert_eq!(
            Some(CharacterSequenceId::Walk),
            Walk::update(CharacterSequenceUpdateComponents::new(
                &input,
                HealthPoints::default(),
                CharacterSequenceId::Walk,
                SequenceStatus::default(),
                &Position::default(),
                &Velocity::default(),
                Mirrored(true),
                Grounding::default(),
                RunCounter::Increase(11)
            ))
        );
    }

    #[test]
    fn walk_when_x_axis_negative_non_mirror() {
        let input = ControllerInput::new(-1., 0., false, false, false, false);

        assert_eq!(
            Some(CharacterSequenceId::Walk),
            Walk::update(CharacterSequenceUpdateComponents::new(
                &input,
                HealthPoints::default(),
                CharacterSequenceId::Walk,
                SequenceStatus::default(),
                &Position::default(),
                &Velocity::default(),
                Mirrored(false),
                Grounding::default(),
                RunCounter::Increase(11)
            ))
        );
    }

    #[test]
    fn walk_when_z_axis_non_zero() {
        let input = ControllerInput::new(0., 1., false, false, false, false);

        assert_eq!(
            None,
            Walk::update(CharacterSequenceUpdateComponents::new(
                &input,
                HealthPoints::default(),
                CharacterSequenceId::Walk,
                SequenceStatus::default(),
                &Position::default(),
                &Velocity::default(),
                Mirrored::default(),
                Grounding::default(),
                RunCounter::Increase(0)
            ))
        );

        let input = ControllerInput::new(0., -1., false, false, false, false);

        assert_eq!(
            None,
            Walk::update(CharacterSequenceUpdateComponents::new(
                &input,
                HealthPoints::default(),
                CharacterSequenceId::Walk,
                SequenceStatus::default(),
                &Position::default(),
                &Velocity::default(),
                Mirrored::default(),
                Grounding::default(),
                RunCounter::Increase(0)
            ))
        );
    }

    #[test]
    fn restarts_walk_when_sequence_ended() {
        vec![(0., 1.), (0., -1.)]
            .into_iter()
            .for_each(|(x_input, z_input)| {
                let input = ControllerInput::new(x_input, z_input, false, false, false, false);

                assert_eq!(
                    Some(CharacterSequenceId::Walk),
                    Walk::update(CharacterSequenceUpdateComponents::new(
                        &input,
                        HealthPoints::default(),
                        CharacterSequenceId::Walk,
                        SequenceStatus::End,
                        &Position::default(),
                        &Velocity::default(),
                        Mirrored(false),
                        Grounding::default(),
                        RunCounter::Increase(0)
                    ))
                );
            });

        vec![(1., 1., false), (-1., -1., true)]
            .into_iter()
            .for_each(|(x_input, z_input, mirrored)| {
                let input = ControllerInput::new(x_input, z_input, false, false, false, false);

                assert_eq!(
                    Some(CharacterSequenceId::Walk),
                    Walk::update(CharacterSequenceUpdateComponents::new(
                        &input,
                        HealthPoints::default(),
                        CharacterSequenceId::Walk,
                        SequenceStatus::End,
                        &Position::default(),
                        &Velocity::default(),
                        mirrored.into(),
                        Grounding::default(),
                        RunCounter::Increase(1)
                    ))
                );
            });
    }

    #[test]
    fn run_when_x_axis_positive_and_run_counter_decrease_non_mirror() {
        let input = ControllerInput::new(1., -1., false, false, false, false);

        assert_eq!(
            Some(CharacterSequenceId::Run),
            Walk::update(CharacterSequenceUpdateComponents::new(
                &input,
                HealthPoints::default(),
                CharacterSequenceId::Walk,
                SequenceStatus::default(),
                &Position::default(),
                &Velocity::default(),
                Mirrored(false),
                Grounding::default(),
                RunCounter::Decrease(10)
            ))
        );
    }

    #[test]
    fn run_when_x_axis_negative_and_run_counter_decrease_mirror() {
        let input = ControllerInput::new(-1., -1., false, false, false, false);

        assert_eq!(
            Some(CharacterSequenceId::Run),
            Walk::update(CharacterSequenceUpdateComponents::new(
                &input,
                HealthPoints::default(),
                CharacterSequenceId::Walk,
                SequenceStatus::default(),
                &Position::default(),
                &Velocity::default(),
                Mirrored(true),
                Grounding::default(),
                RunCounter::Decrease(10)
            ))
        );
    }
}