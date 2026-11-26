use ferrum_ui::components::accordion::{AccordionItem, AccordionProps, AccordionState};
use ferrum_ui::components::alert::AlertProps;
use ferrum_ui::components::avatar::AvatarProps;
use ferrum_ui::components::badge::BadgeProps;
use ferrum_ui::components::button::ButtonProps;
use ferrum_ui::components::card::CardProps;
use ferrum_ui::components::checkbox::{CheckboxProps, CheckboxState};
use ferrum_ui::components::dialog::{DialogController, DialogProps};
use ferrum_ui::components::input::{InputProps, InputState};
use ferrum_ui::components::menu::{MenuItem, MenuProps, MenuState};
use ferrum_ui::components::progress::ProgressProps;
use ferrum_ui::components::separator::SeparatorProps;
use ferrum_ui::components::skeleton::SkeletonProps;
use ferrum_ui::components::tabs::{Tab, TabsProps, TabsState};
use ferrum_ui::components::toast::{Toast, ToastQueue};
use ferrum_ui_ssr::{
    accordion, alert, avatar, badge, button, card, checkbox, dialog, input, menu, preview_page,
    progress, separator, skeleton, tabs, toast_queue,
};

#[test]
fn preview_page_inlines_light_and_dark_tokens() {
    let html = preview_page("Preview", "<p>Body</p>");
    assert!(html.contains("--bg: 0 0% 100%;"));
    assert!(html.contains("prefers-color-scheme: dark"));
    assert!(html.contains("<p>Body</p>"));
}

#[test]
fn button_and_dialog_render_semantic_markup() {
    let button_html = button(&ButtonProps::new("Save"));
    assert!(button_html.contains("<button"));
    assert!(button_html.contains("Save"));

    let props = DialogProps {
        title: "Settings".to_owned(),
        open: true,
        ..DialogProps::default()
    };
    let mut controller = DialogController::new();
    controller.open(None, vec!["close".into()]);
    let dialog_html = dialog(&props, &controller);
    assert!(dialog_html.contains("role=\"dialog\""));
    assert!(dialog_html.contains("aria-modal=\"true\""));
}

#[test]
fn menu_renders_only_focusable_items_with_roving_tabindex() {
    let props = MenuProps {
        items: vec![
            MenuItem::item("new", "New"),
            MenuItem::disabled("delete", "Delete"),
            MenuItem::item("open", "Open"),
        ],
        open: true,
        ..MenuProps::default()
    };
    let state = MenuState::new(&props);
    let html = menu(&props, &state);
    assert!(html.contains("role=\"menu\""));
    assert!(html.contains("tabindex=\"0\""));
    assert!(html.contains("aria-disabled=\"true\""));
}

#[test]
fn tabs_and_toast_render_regions() {
    let props = TabsProps {
        tabs: vec![Tab::new("a", "A", "Panel A")],
        ..TabsProps::default()
    };
    let state = TabsState::new(&props);
    assert!(tabs(&props, &state).contains("role=\"tablist\""));

    let mut queue = ToastQueue::new();
    queue.push(Toast::new("saved", "Saved"));
    let html = toast_queue(&queue);
    assert!(html.contains("aria-live=\"polite\""));
    assert!(html.contains("Saved"));
}

#[test]
fn phase_four_components_render_semantic_markup() {
    let accordion_props = AccordionProps {
        items: vec![AccordionItem::new("a", "A", "Panel A")],
        default_open: vec![0],
        ..AccordionProps::default()
    };
    let accordion_state = AccordionState::new(&accordion_props);
    assert!(accordion(&accordion_props, &accordion_state).contains("aria-expanded=\"true\""));

    assert!(alert(&AlertProps::new("Saved")).contains("role=\"alert\""));
    assert!(avatar(&AvatarProps::new("AB")).contains("role=\"img\""));
    assert!(badge(&BadgeProps::new("New")).contains("ferrum-badge"));
    assert!(card(&CardProps::new("Project")).contains("<h2>Project</h2>"));

    let checkbox_props = CheckboxProps::new("Accept");
    let checkbox_state = CheckboxState::new(&checkbox_props);
    assert!(checkbox(&checkbox_props, &checkbox_state).contains("type=\"checkbox\""));

    let input_props = InputProps::new("Email");
    let input_state = InputState::new(&input_props);
    let input_html = input(&input_props, &input_state);
    assert!(input_html.contains("type=\"text\""));
    assert!(input_html.contains("Email"));

    assert!(progress(&ProgressProps::new(40)).contains("role=\"progressbar\""));
    assert!(separator(&SeparatorProps::new()).contains("role=\"separator\""));
    assert!(skeleton(&SkeletonProps::new()).contains("aria-hidden=\"true\""));
}

#[test]
fn every_promoted_component_has_preview_a11y_metadata() {
    let accordion_props = AccordionProps {
        items: vec![AccordionItem::new("a", "A", "Panel A")],
        default_open: vec![0],
        ..AccordionProps::default()
    };
    let accordion_state = AccordionState::new(&accordion_props);
    let checkbox_props = CheckboxProps::new("Accept");
    let checkbox_state = CheckboxState::new(&checkbox_props);
    let input_props = InputProps::new("Email");
    let input_state = InputState::new(&input_props);
    let mut queue = ToastQueue::new();
    queue.push(Toast::new("saved", "Saved"));

    let cases: [(&str, String, &str); 15] = [
        (
            "accordion",
            accordion(&accordion_props, &accordion_state),
            "aria-expanded",
        ),
        ("alert", alert(&AlertProps::new("Saved")), "role=\"alert\""),
        ("avatar", avatar(&AvatarProps::new("AB")), "role=\"img\""),
        ("badge", badge(&BadgeProps::new("New")), "ferrum-badge"),
        ("button", button(&ButtonProps::new("Save")), "<button"),
        ("card", card(&CardProps::new("Project")), "<h2"),
        (
            "checkbox",
            checkbox(&checkbox_props, &checkbox_state),
            "type=\"checkbox\"",
        ),
        (
            "dialog",
            dialog(
                &DialogProps {
                    open: true,
                    ..DialogProps::default()
                },
                &{
                    let mut controller = DialogController::new();
                    controller.open(None, vec!["close".into()]);
                    controller
                },
            ),
            "role=\"dialog\"",
        ),
        ("input", input(&input_props, &input_state), "<label"),
        (
            "menu",
            menu(
                &MenuProps {
                    items: vec![MenuItem::item("new", "New")],
                    open: true,
                    ..MenuProps::default()
                },
                &MenuState::new(&MenuProps {
                    items: vec![MenuItem::item("new", "New")],
                    open: true,
                    ..MenuProps::default()
                }),
            ),
            "role=\"menu\"",
        ),
        (
            "progress",
            progress(&ProgressProps::new(40)),
            "role=\"progressbar\"",
        ),
        (
            "separator",
            separator(&SeparatorProps::new()),
            "role=\"separator\"",
        ),
        ("skeleton", skeleton(&SkeletonProps::new()), "aria-hidden"),
        (
            "tabs",
            tabs(
                &TabsProps {
                    tabs: vec![Tab::new("a", "A", "Panel A")],
                    ..TabsProps::default()
                },
                &TabsState::new(&TabsProps {
                    tabs: vec![Tab::new("a", "A", "Panel A")],
                    ..TabsProps::default()
                }),
            ),
            "role=\"tablist\"",
        ),
        ("toast", toast_queue(&queue), "aria-live"),
    ];

    for (name, html, marker) in cases {
        assert!(!html.is_empty(), "{name} preview is empty");
        assert!(
            html.contains(marker),
            "{name} preview missing {marker}: {html}"
        );
    }
}
