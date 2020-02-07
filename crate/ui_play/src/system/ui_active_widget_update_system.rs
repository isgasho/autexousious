use amethyst::{
    ecs::{Read, ReadStorage, System, World, WriteStorage},
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
};
use derivative::Derivative;
use derive_new::new;
use game_input_model::play::{AxisMoveEventData, ControlInputEvent, MoveDirection};
use ui_model_spi::{
    config::WidgetStatus,
    play::{Siblings, SiblingsVertical},
};

/// Updates `WidgetStatus` based on `ControlInputEvent`s and `Sibling`s.
#[derive(Debug, Default, new)]
pub struct UiActiveWidgetUpdateSystem {
    /// Reader ID for the `ControlInputEvent` channel.
    #[new(default)]
    control_input_event_rid: Option<ReaderId<ControlInputEvent>>,
}

/// `UiActiveWidgetInputResources`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct UiActiveWidgetInputResources<'s> {
    /// `WidgetStatus` components.
    #[derivative(Debug = "ignore")]
    pub widget_statuses: WriteStorage<'s, WidgetStatus>,
    /// `Siblings` components.
    #[derivative(Debug = "ignore")]
    pub siblingses: ReadStorage<'s, Siblings>,
    /// `SiblingsVertical` components.
    #[derivative(Debug = "ignore")]
    pub siblings_verticals: ReadStorage<'s, SiblingsVertical>,
}

/// `UiActiveWidgetUpdateSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct UiActiveWidgetUpdateSystemData<'s> {
    /// `ControlInputEvent` channel.
    #[derivative(Debug = "ignore")]
    pub control_input_ec: Read<'s, EventChannel<ControlInputEvent>>,
    /// `UiActiveWidgetInputResources`.
    pub ui_active_widget_input_resources: UiActiveWidgetInputResources<'s>,
}

impl UiActiveWidgetUpdateSystem {
    fn handle_event(
        UiActiveWidgetInputResources {
            ref mut widget_statuses,
            ref siblingses,
            ref siblings_verticals,
        }: &mut UiActiveWidgetInputResources,
        axis_move_event_data: AxisMoveEventData,
    ) -> bool {
        let ui_entity = axis_move_event_data.entity;
        let widget_status = widget_statuses.get(ui_entity).copied();

        let move_direction = MoveDirection::from(axis_move_event_data);
        let ui_entity_sibling = match move_direction {
            MoveDirection::None => None,
            MoveDirection::Up => siblings_verticals
                .get(ui_entity)
                .and_then(|siblings_vertical| siblings_vertical.up),
            MoveDirection::Down => siblings_verticals
                .get(ui_entity)
                .and_then(|siblings_vertical| siblings_vertical.down),
            MoveDirection::Left => siblingses
                .get(ui_entity)
                .and_then(|siblings| siblings.previous),
            MoveDirection::Right => siblingses.get(ui_entity).and_then(|siblings| siblings.next),
        };

        if let (Some(WidgetStatus::Active), Some(ui_entity_sibling)) =
            (widget_status, ui_entity_sibling)
        {
            widget_statuses
                .insert(ui_entity, WidgetStatus::Idle)
                .expect("Failed to insert `WidgetStatus` component.");
            widget_statuses
                .insert(ui_entity_sibling, WidgetStatus::Active)
                .expect("Failed to insert `WidgetStatus` component.");

            true
        } else {
            false
        }
    }
}

impl<'s> System<'s> for UiActiveWidgetUpdateSystem {
    type SystemData = UiActiveWidgetUpdateSystemData<'s>;

    fn run(
        &mut self,
        UiActiveWidgetUpdateSystemData {
            control_input_ec,
            mut ui_active_widget_input_resources,
        }: Self::SystemData,
    ) {
        let control_input_event_rid = self
            .control_input_event_rid
            .as_mut()
            .expect("Expected `control_input_event_rid` field to be set.");

        // Ignore result as it is only used for the quick exit.
        let _ = control_input_ec
            .read(control_input_event_rid)
            .copied()
            .try_for_each(|ev| {
                if let ControlInputEvent::AxisMoved(axis_move_event_data) = ev {
                    let event_handled = Self::handle_event(
                        &mut ui_active_widget_input_resources,
                        axis_move_event_data,
                    );
                    if event_handled {
                        Err(())
                    } else {
                        Ok(())
                    }
                } else {
                    Ok(())
                }
            });
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);

        self.control_input_event_rid = Some(
            world
                .fetch_mut::<EventChannel<ControlInputEvent>>()
                .register_reader(),
        );
    }
}
