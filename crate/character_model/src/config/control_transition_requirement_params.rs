use charge_model::{config::ChargeUseMode, play::ChargeTrackerClock};
use game_input::ControllerInput;
use object_model::play::{HealthPoints, Mirrored, SkillPoints};

/// Parameters to check if a `ControlTransitionRequirement` is met.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ControlTransitionRequirementParams {
    /// `HealthPoints` of the entity.
    pub health_points: Option<HealthPoints>,
    /// `SkillPoints` of the entity.
    pub skill_points: Option<SkillPoints>,
    /// `ChargeTrackerClock` of the entity.
    pub charge_tracker_clock: Option<ChargeTrackerClock>,
    /// `ChargeUseMode` of the entity.
    pub charge_use_mode: Option<ChargeUseMode>,
    /// `ControllerInput` of the entity.
    pub controller_input: Option<ControllerInput>,
    /// `Mirrored` of the entity.
    pub mirrored: Option<Mirrored>,
}
