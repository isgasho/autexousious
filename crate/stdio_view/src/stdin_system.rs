use std::{
    sync::mpsc::{self, Receiver, TryRecvError},
    thread,
};

use amethyst::{ecs::prelude::*, shrev::EventChannel};
use application_input::ApplicationEvent;
use log::{debug, error, trace, warn};
use stdio_spi::VariantAndTokens;
use typename_derive::TypeName;

use crate::{
    reader::{self, StdinReader},
    IoAppEventUtils,
};

/// Type to fetch the application event channel.
type StdinSystemData<'s> = (
    Write<'s, EventChannel<ApplicationEvent>>,
    Write<'s, EventChannel<VariantAndTokens>>,
);

/// Rendering system.
#[derive(Debug, TypeName)]
pub struct StdinSystem {
    /// Channel receiver for output/input messages for this system.
    rx: Receiver<String>,
}

impl StdinSystem {
    /// Returns a new StdinSystem that listens to stdin on a separate thread.
    // kcov-ignore-start
    pub fn new() -> Self {
        // kcov-ignore-end
        Self::default()
    }

    /// Returns a new StdinSystem
    ///
    /// Allows tests to retain control over the channel sender.
    fn internal_new<F>(rx: Receiver<String>, reader_spawn_fn: F) -> Self
    where
        F: FnOnce(),
    {
        reader_spawn_fn();
        StdinSystem { rx }
    }
}

impl Default for StdinSystem {
    fn default() -> Self {
        let (tx, rx) = mpsc::channel();
        let reader_spawn_fn = || {
            thread::Builder::new()
                .name(reader::NAME.to_string())
                .spawn(|| StdinReader::new(tx).start())
                // TODO: replace new() with build() and return Result<..>
                .expect("Failed to spawn StdinReader thread.");
        };
        Self::internal_new(rx, reader_spawn_fn)
    } // kcov-ignore
}

impl<'s> System<'s> for StdinSystem {
    type SystemData = StdinSystemData<'s>;

    fn run(&mut self, (mut application_event_channel, mut variant_channel): Self::SystemData) {
        match self.rx.try_recv() {
            Ok(input) => {
                debug!("Input from StdinReader: `{:?}`.", &input);

                if input == "exit" {
                    application_event_channel.single_write(ApplicationEvent::Exit);
                    return;
                }

                match IoAppEventUtils::input_to_variant_and_tokens(&input) {
                    Ok(variant_and_tokens) => {
                        if let Some(variant_and_tokens) = variant_and_tokens {
                            variant_channel.single_write(variant_and_tokens);
                        }
                    }
                    Err(e) => error!("Failed to parse input. Error: `{}`.", e),
                }
            }
            Err(TryRecvError::Empty) => {
                // do nothing
                trace!("No message from StdinReader");
            }
            Err(TryRecvError::Disconnected) => {
                warn!("Channel receiver to `StdinReader` disconnected.");
            }
        };
    }
}

#[cfg(test)]
mod test {
    use std::sync::mpsc::{self, Sender};

    use amethyst::{
        ecs::prelude::RunNow,
        shred::{Resources, SystemData},
        shrev::{EventChannel, ReaderId},
    };
    use application_event::AppEventVariant;
    use application_input::ApplicationEvent;

    use super::{StdinSystem, StdinSystemData};
    use stdio_spi::VariantAndTokens;

    fn setup() -> (
        StdinSystem,
        Sender<String>,
        Resources,
        ReaderId<ApplicationEvent>,
        ReaderId<VariantAndTokens>,
    ) {
        let mut res = Resources::new();
        res.insert(EventChannel::<ApplicationEvent>::with_capacity(10));
        res.insert(EventChannel::<VariantAndTokens>::with_capacity(10));

        let (tx, rx) = mpsc::channel();
        let stdin_system = StdinSystem::internal_new(rx, || {});

        let (application_ev_id, variant_and_tokens_id) = {
            let (mut application_events, mut variant_and_tokens) = StdinSystemData::fetch(&res);
            (
                application_events.register_reader(),
                variant_and_tokens.register_reader(),
            ) // kcov-ignore
        }; // kcov-ignore

        (
            stdin_system,
            tx,
            res,
            application_ev_id,
            variant_and_tokens_id,
        )
    }

    #[test]
    fn sends_exit_event_when_input_is_exit() {
        let (mut stdin_system, tx, res, mut application_ev_id, _) = setup();

        tx.send("exit".to_string()).unwrap();
        stdin_system.run_now(&res);

        let (application_events, _) = StdinSystemData::fetch(&res);

        expect_event(
            &application_events,
            &mut application_ev_id,
            Some(&ApplicationEvent::Exit),
        );
    } // kcov-ignore

    #[test]
    fn does_not_send_exit_event_when_input_is_not_exit() {
        let (mut stdin_system, tx, res, mut application_ev_id, _) = setup();

        tx.send("abc".to_string()).unwrap();
        stdin_system.run_now(&res);

        let (application_events, _) = StdinSystemData::fetch(&res);
        expect_event(&application_events, &mut application_ev_id, None);
    } // kcov-ignore

    #[test]
    fn does_nothing_when_input_is_empty() {
        let (mut stdin_system, _tx, res, mut application_ev_id, _) = setup();

        // we don't call tx.send(..)
        stdin_system.run_now(&res);

        let (application_events, _) = StdinSystemData::fetch(&res);
        expect_event(&application_events, &mut application_ev_id, None);
    } // kcov-ignore

    #[test]
    fn does_not_panic_when_application_channel_is_disconnected() {
        let (mut stdin_system, tx, res, mut application_ev_id, _) = setup();

        drop(tx); // ensure channel is disconnected
        stdin_system.run_now(&res);

        let (application_events, _) = StdinSystemData::fetch(&res);
        expect_event(&application_events, &mut application_ev_id, None);
    } // kcov-ignore

    #[test]
    fn sends_vat_event_when_input_is_app_event() {
        let (mut stdin_system, tx, res, _, mut vat_ev_id) = setup();

        tx.send("character_selection confirm".to_string()).unwrap();
        stdin_system.run_now(&res);

        let (_, vat_events) = StdinSystemData::fetch(&res);

        expect_vat_event(
            &vat_events,
            &mut vat_ev_id,
            Some(&(
                AppEventVariant::CharacterSelection,
                vec!["character_selection".to_string(), "confirm".to_string()],
            )),
        ); // kcov-ignore
    }

    fn expect_event(
        application_events: &EventChannel<ApplicationEvent>,
        mut application_ev_id: &mut ReaderId<ApplicationEvent>,
        expected_event: Option<&ApplicationEvent>,
    ) {
        let mut event_it = application_events.read(&mut application_ev_id);
        assert_eq!(expected_event, event_it.next());
    }

    fn expect_vat_event(
        vat_events: &EventChannel<VariantAndTokens>,
        mut vat_ev_id: &mut ReaderId<VariantAndTokens>,
        expected_event: Option<&VariantAndTokens>,
    ) {
        let mut event_it = vat_events.read(&mut vat_ev_id);
        assert_eq!(expected_event, event_it.next());
    }
}
