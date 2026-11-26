use ferrum_ui::components::accordion::{
    AccordionAction, AccordionItem, AccordionProps, AccordionState,
};
use ferrum_ui::components::alert::{AlertProps, AlertVariant};
use ferrum_ui::components::avatar::{AvatarProps, AvatarSize};
use ferrum_ui::components::badge::{BadgeProps, BadgeVariant};
use ferrum_ui::components::button::{ButtonProps, ButtonSize, ButtonVariant};
use ferrum_ui::components::card::CardProps;
use ferrum_ui::components::checkbox::{CheckboxAction, CheckboxProps, CheckboxState};
use ferrum_ui::components::dialog::{DialogAction, DialogController, DialogProps};
use ferrum_ui::components::input::{InputAction, InputKind, InputProps, InputState};
use ferrum_ui::components::menu::{MenuAction, MenuItem, MenuProps, MenuState};
use ferrum_ui::components::progress::ProgressProps;
use ferrum_ui::components::separator::{SeparatorOrientation, SeparatorProps};
use ferrum_ui::components::skeleton::SkeletonProps;
use ferrum_ui::components::tabs::{Tab, TabActivation, TabsAction, TabsProps, TabsState};
use ferrum_ui::components::toast::{Toast, ToastQueue, ToastVariant};
use ferrum_ui_core::event::{Key, KeyEvent};

#[test]
fn button_defaults_and_variants_are_accessible() {
    let props = ButtonProps::new("Continue")
        .with_variant(ButtonVariant::Outline)
        .with_size(ButtonSize::Lg)
        .disabled(true);
    let html = props.element().render();
    assert!(html.contains("ferrum-button--outline"));
    assert!(html.contains("ferrum-button--lg"));
    assert!(html.contains("aria-disabled=\"true\""));
    assert!(html.contains("disabled=\"\""));
}

#[test]
fn dialog_traps_focus_and_restores_trigger() {
    let mut controller = DialogController::new();
    controller.open(
        Some("open-dialog".into()),
        vec!["first".into(), "last".into()],
    );
    let props = DialogProps::default();
    assert_eq!(
        controller.handle_key(&props, &KeyEvent::new(Key::Tab)),
        DialogAction::PreventDefault
    );
    assert_eq!(controller.focus.active(), Some("last"));
    assert_eq!(
        controller.handle_key(&props, &KeyEvent::new(Key::Escape)),
        DialogAction::Closed
    );
    assert_eq!(
        controller.state.restore_focus.as_deref(),
        Some("open-dialog")
    );
}

#[test]
fn menu_typeahead_and_escape_work() {
    let props = MenuProps {
        items: vec![
            MenuItem::item("new", "New task"),
            MenuItem::disabled("delete", "Delete"),
            MenuItem::item("open", "Open"),
        ],
        open: true,
        ..MenuProps::default()
    };
    let mut state = MenuState::new(&props);
    assert_eq!(
        state.handle_key(&props, &KeyEvent::new(Key::Char('o'))),
        MenuAction::Moved(Some("open".into()))
    );
    assert_eq!(
        state.handle_key(&props, &KeyEvent::new(Key::Escape)),
        MenuAction::Closed
    );
}

#[test]
fn tabs_support_manual_activation() {
    let props = TabsProps {
        tabs: vec![Tab::new("a", "A", "Panel A"), Tab::new("b", "B", "Panel B")],
        activation: TabActivation::Manual,
        ..TabsProps::default()
    };
    let mut state = TabsState::new(&props);
    assert_eq!(
        state.handle_key(&props, &KeyEvent::new(Key::ArrowRight)),
        TabsAction::Focused(1)
    );
    assert_eq!(state.active, 0);
    assert_eq!(state.focus, 1);
}

#[test]
fn toast_queue_dedupes_and_switches_live_region() {
    let mut queue = ToastQueue::new();
    assert!(queue.push(Toast::new("saved", "Saved")));
    assert!(!queue.push(
        Toast::new("saved", "Saved")
            .with_variant(ToastVariant::Success)
            .with_duration(1_000)
    ));
    assert_eq!(queue.len(), 1);
    queue.pause();
    assert!(queue.aria().to_string().contains("aria-live=\"off\""));
}

#[test]
fn accordion_and_checkbox_keyboard_models_work() {
    let accordion_props = AccordionProps {
        items: vec![
            AccordionItem::new("a", "A", "Panel A"),
            AccordionItem::new("b", "B", "Panel B"),
        ],
        ..AccordionProps::default()
    };
    let mut accordion = AccordionState::new(&accordion_props);
    assert_eq!(
        accordion.handle_key(&accordion_props, &KeyEvent::new(Key::ArrowDown)),
        AccordionAction::Focused(1)
    );

    let checkbox_props = CheckboxProps::new("Accept");
    let mut checkbox = CheckboxState::new(&checkbox_props);
    assert_eq!(
        checkbox.handle_key(&checkbox_props, &KeyEvent::new(Key::Space)),
        CheckboxAction::Changed(true)
    );
}

#[test]
fn phase_four_models_are_accessible() {
    assert!(AlertProps::new("Saved")
        .with_variant(AlertVariant::Danger)
        .element()
        .render()
        .contains("ferrum-alert--danger"));
    assert!(AvatarProps::new("AB")
        .with_size(AvatarSize::Lg)
        .element()
        .render()
        .contains("aria-label=\"AB\""));
    assert!(BadgeProps::new("New")
        .with_variant(BadgeVariant::Outline)
        .element()
        .render()
        .contains("ferrum-badge--outline"));
    assert!(CardProps::new("Project")
        .with_footer("Done")
        .element()
        .render()
        .contains("ferrum-card-footer"));

    let input_props = InputProps::new("Email")
        .with_kind(InputKind::Email)
        .invalid(true);
    let mut input = InputState::new(&input_props);
    assert_eq!(
        input.handle_key(&input_props, &KeyEvent::new(Key::Char('a'))),
        InputAction::Changed("a".into())
    );
    assert!(input_props
        .element(&input)
        .render()
        .contains("aria-invalid=\"true\""));

    assert!(ProgressProps::new(50)
        .element()
        .render()
        .contains("aria-valuenow=\"50\""));
    assert!(SeparatorProps::new()
        .vertical()
        .element()
        .render()
        .contains("aria-orientation=\"vertical\""));
    assert_eq!(SeparatorOrientation::Vertical.as_str(), "vertical");
    assert!(SkeletonProps::new()
        .element()
        .render()
        .contains("aria-hidden=\"true\""));
}
