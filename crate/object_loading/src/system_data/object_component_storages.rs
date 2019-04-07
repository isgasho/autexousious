use amethyst::{
    core::Transform,
    ecs::WriteStorage,
    renderer::{Flipped, Transparent},
};
use derivative::Derivative;
use object_model::play::{Mirrored, Position, Velocity};
use sequence_model::{
    config::SequenceId,
    loaded::{ComponentSequencesHandle, SequenceEndTransitions},
    play::{FrameIndexClock, FrameWaitClock, SequenceStatus},
};
use shred_derive::SystemData;

/// Common game object `Component` storages.
///
/// These are the storages for the components common to all game objects.
///
/// # Type Parameters:
///
/// * `SeqId`: Sequence ID of the object, such as `CharacterSequenceId`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct ObjectComponentStorages<'s, SeqId>
where
    SeqId: SequenceId + 'static,
{
    /// `Flipped` component storage.
    #[derivative(Debug = "ignore")]
    pub flippeds: WriteStorage<'s, Flipped>,
    /// `Transparent` component storage.
    #[derivative(Debug = "ignore")]
    pub transparents: WriteStorage<'s, Transparent>,
    /// `Position` component storage.
    #[derivative(Debug = "ignore")]
    pub positions: WriteStorage<'s, Position<f32>>,
    /// `Velocity` component storage.
    #[derivative(Debug = "ignore")]
    pub velocities: WriteStorage<'s, Velocity<f32>>,
    /// `Transform` component storage.
    #[derivative(Debug = "ignore")]
    pub transforms: WriteStorage<'s, Transform>,
    /// `Mirrored` component storage.
    #[derivative(Debug = "ignore")]
    pub mirroreds: WriteStorage<'s, Mirrored>,
    /// `ComponentSequencesHandle` component storage.
    #[derivative(Debug = "ignore")]
    pub component_sequences_handles: WriteStorage<'s, ComponentSequencesHandle>,
    /// `SequenceEndTransitions` component storage.
    #[derivative(Debug = "ignore")]
    pub sequence_end_transitionses: WriteStorage<'s, SequenceEndTransitions<SeqId>>,
    /// `SeqId` component storage.
    #[derivative(Debug = "ignore")]
    pub sequence_ids: WriteStorage<'s, SeqId>,
    /// `SequenceStatus` component storage.
    #[derivative(Debug = "ignore")]
    pub sequence_statuses: WriteStorage<'s, SequenceStatus>,
    /// `FrameIndexClock` component storage.
    #[derivative(Debug = "ignore")]
    pub frame_index_clocks: WriteStorage<'s, FrameIndexClock>,
    /// `FrameWaitClock` component storage.
    #[derivative(Debug = "ignore")]
    pub frame_wait_clocks: WriteStorage<'s, FrameWaitClock>,
}
