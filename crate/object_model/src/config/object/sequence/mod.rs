//! Configuration types for object sequences.
//!
//! A sequence is an independent grouping of frames, which contains not only animation information,
//! but also the collision zones, interaction, and effects.
//!
//! Sequences are shared by different object types, and are genericized by the sequence ID. This is
//! because different object types have different valid sequence IDs, and we want to be able to
//! define this at compile time rather than needing to process this at run time.

pub use self::frame::Frame;
pub use self::sequence_id::SequenceId;
pub use self::sequence_state::SequenceState;

mod frame;
mod sequence_id;
mod sequence_state;

/// Represents an independent action sequence of an object.
///
/// This carries the information necessary for an `Animation`, as well as the effects and
/// interactions that happen during each frame of that animation.
#[derive(Clone, Debug, Deserialize, PartialEq, new)]
pub struct Sequence<SeqId: SequenceId> {
    /// ID of the sequence to switch to after this one has completed.
    ///
    /// Note: This may not be immediately after the last frame of the sequence. For example, a
    /// character that is in mid-air should remain in the last frame until it lands on the ground.
    pub next: Option<SeqId>,
    /// Key frames in the animation sequence.
    pub frames: Vec<Frame>,
}

#[cfg(test)]
mod test {
    use toml;

    use super::{Frame, Sequence, SequenceId};

    const SEQUENCE_TOML: &str = r#"
        next = "Boo"
        frames = [
          { sheet = 0, sprite = 4, wait = 2 },
          { sheet = 0, sprite = 5, wait = 2 },
          { sheet = 1, sprite = 6, wait = 1 },
          { sheet = 1, sprite = 7, wait = 1 },
          { sheet = 0, sprite = 6, wait = 2 },
          { sheet = 0, sprite = 5, wait = 2 },
        ]
    "#;
    const SEQUENCE_TOML_2: &str = r#"
        frames = []
    "#;

    #[test]
    fn deserialize_sequence() {
        let sequence = toml::from_str::<Sequence<TestSeqId>>(SEQUENCE_TOML)
            .expect("Failed to deserialize sequence.");

        let frames = vec![
            Frame::new(0, 4, 2),
            Frame::new(0, 5, 2),
            Frame::new(1, 6, 1),
            Frame::new(1, 7, 1),
            Frame::new(0, 6, 2),
            Frame::new(0, 5, 2),
        ];
        let expected = Sequence::new(Some(TestSeqId::Boo), frames);
        assert_eq!(expected, sequence);
    }

    #[test]
    fn deserialize_empty_sequence() {
        let sequence = toml::from_str::<Sequence<TestSeqId>>(SEQUENCE_TOML_2)
            .expect("Failed to deserialize sequence.");

        let expected = Sequence::new(None, vec![]);
        assert_eq!(expected, sequence);
    }

    #[derive(Clone, Copy, Debug, Derivative, Deserialize, PartialEq, Eq, Hash)]
    #[derivative(Default)]
    enum TestSeqId {
        #[derivative(Default)]
        Boo,
    }
    impl SequenceId for TestSeqId {}
}
