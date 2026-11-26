use std::fs;
use std::path::Path;

use ferrum_ui::components::accordion::{AccordionItem, AccordionProps, AccordionState};
use ferrum_ui::components::alert::AlertProps;
use ferrum_ui::components::avatar::{AvatarProps, AvatarSize};
use ferrum_ui::components::badge::{BadgeProps, BadgeVariant};
use ferrum_ui::components::breadcrumb::BreadcrumbProps;
use ferrum_ui::components::button::{ButtonProps, ButtonSize, ButtonVariant};
use ferrum_ui::components::card::CardProps;
use ferrum_ui::components::checkbox::{CheckboxProps, CheckboxState};
use ferrum_ui::components::dialog::{DialogController, DialogProps};
use ferrum_ui::components::empty_state::EmptyStateProps;
use ferrum_ui::components::input::{InputKind, InputProps, InputState};
use ferrum_ui::components::kbd::KbdProps;
use ferrum_ui::components::label::LabelProps;
use ferrum_ui::components::menu::{MenuItem, MenuProps, MenuState};
use ferrum_ui::components::progress::ProgressProps;
use ferrum_ui::components::radio_group::{RadioGroupProps, RadioGroupState, RadioOption};
use ferrum_ui::components::separator::SeparatorProps;
use ferrum_ui::components::skeleton::SkeletonProps;
use ferrum_ui::components::slider::SliderProps;
use ferrum_ui::components::spinner::SpinnerProps;
use ferrum_ui::components::switch::{SwitchProps, SwitchState};
use ferrum_ui::components::tabs::{Tab, TabsProps, TabsState};
use ferrum_ui::components::textarea::{TextareaProps, TextareaState};
use ferrum_ui::components::toast::{Toast, ToastQueue};
use ferrum_ui::components::tooltip::TooltipProps;
use ferrum_ui_ssr::{
    accordion, alert, avatar, badge, breadcrumb, button, card, checkbox, dialog, empty_state,
    input, kbd, label, menu, preview_page, progress, radio_group, separator, skeleton, slider,
    spinner, switch, tabs, textarea, toast_queue, tooltip,
};

fn main() {
    let out = Path::new("target/previews");
    fs::create_dir_all(out).expect("create preview dir");

    let button_props = ButtonProps::new("Save changes")
        .with_variant(ButtonVariant::Default)
        .with_size(ButtonSize::Default);
    let dialog_props = DialogProps {
        title: "New task".to_owned(),
        description: "Create a task in the current project.".to_owned(),
        open: true,
        ..DialogProps::default()
    };
    let mut controller = DialogController::new();
    controller.open(
        Some("new-task".to_owned()),
        vec!["cancel".to_owned(), "create".to_owned()],
    );
    let menu_props = MenuProps {
        items: vec![
            MenuItem::item("new", "New task").with_shortcut("⌘N"),
            MenuItem::disabled("delete", "Delete"),
            MenuItem::separator("sep"),
            MenuItem::item("open", "Open"),
        ],
        open: true,
        ..MenuProps::default()
    };
    let menu_state = MenuState::new(&menu_props);
    let tabs_props = TabsProps {
        tabs: vec![
            Tab::new("board", "Board", "Board panel"),
            Tab::new("activity", "Activity", "Activity panel"),
        ],
        ..TabsProps::default()
    };
    let tabs_state = TabsState::new(&tabs_props);
    let mut queue = ToastQueue::new();
    queue.push(Toast::new("saved", "Changes saved").with_description("Synced just now"));

    let accordion_props = AccordionProps {
        items: vec![
            AccordionItem::new("one", "Is it accessible?", "Yes. It uses semantic buttons."),
            AccordionItem::new("two", "Is it styled?", "Yes. Tokens drive every theme."),
        ],
        default_open: vec![0],
        ..AccordionProps::default()
    };
    let accordion_state = AccordionState::new(&accordion_props);
    let alert_props = AlertProps::new("Changes saved").with_description("Synced just now.");
    let avatar_props = AvatarProps::new("AB").with_size(AvatarSize::Md);
    let badge_props = BadgeProps::new("New").with_variant(BadgeVariant::Success);
    let card_props = CardProps::new("Project").with_description("Ship it");
    let checkbox_props = CheckboxProps::new("Accept terms");
    let checkbox_state = CheckboxState::new(&checkbox_props);
    let input_props = InputProps::new("Email")
        .with_placeholder("you@example.com")
        .with_kind(InputKind::Email);
    let input_state = InputState::new(&input_props);
    let progress_props = ProgressProps::new(64).with_label("Upload");
    let separator_props = SeparatorProps::new();
    let skeleton_props = SkeletonProps::new().with_size("180px", "12px");

    let breadcrumb_props = BreadcrumbProps::new(vec!["Home".into(), "Components".into()]);
    let empty_state_props =
        EmptyStateProps::new("No results").with_description("Try another filter");
    let kbd_props = KbdProps::new("⌘K");
    let label_props = LabelProps::new("Email").with_for("email");
    let radio_props = RadioGroupProps {
        name: "size".into(),
        options: vec![
            RadioOption::new("sm", "Small"),
            RadioOption::new("md", "Medium"),
        ],
        value: Some("md".into()),
        ..RadioGroupProps::default()
    };
    let radio_state = RadioGroupState::new(&radio_props);
    let slider_props = SliderProps::new(64);
    let switch_props = SwitchProps::new("Dark mode");
    let switch_state = SwitchState::new(&switch_props);
    let textarea_props = TextareaProps::new("Bio");
    let textarea_state = TextareaState::new(&textarea_props);
    let tooltip_props = TooltipProps::new("More info");
    let spinner_props = SpinnerProps::new();

    let pages = [
        (
            "index",
            preview_page("Ferrum UI previews", "<h1>Ferrum UI</h1>"),
        ),
        ("button", preview_page("Button", &button(&button_props))),
        (
            "dialog",
            preview_page("Dialog", &dialog(&dialog_props, &controller)),
        ),
        (
            "menu",
            preview_page("Menu", &menu(&menu_props, &menu_state)),
        ),
        (
            "tabs",
            preview_page("Tabs", &tabs(&tabs_props, &tabs_state)),
        ),
        ("toast", preview_page("Toast", &toast_queue(&queue))),
        (
            "accordion",
            preview_page("Accordion", &accordion(&accordion_props, &accordion_state)),
        ),
        ("alert", preview_page("Alert", &alert(&alert_props))),
        ("avatar", preview_page("Avatar", &avatar(&avatar_props))),
        ("badge", preview_page("Badge", &badge(&badge_props))),
        ("card", preview_page("Card", &card(&card_props))),
        (
            "checkbox",
            preview_page("Checkbox", &checkbox(&checkbox_props, &checkbox_state)),
        ),
        (
            "input",
            preview_page("Input", &input(&input_props, &input_state)),
        ),
        (
            "progress",
            preview_page("Progress", &progress(&progress_props)),
        ),
        (
            "separator",
            preview_page("Separator", &separator(&separator_props)),
        ),
        (
            "skeleton",
            preview_page("Skeleton", &skeleton(&skeleton_props)),
        ),
        (
            "breadcrumb",
            preview_page("Breadcrumb", &breadcrumb(&breadcrumb_props)),
        ),
        (
            "empty-state",
            preview_page("Empty state", &empty_state(&empty_state_props)),
        ),
        ("kbd", preview_page("Kbd", &kbd(&kbd_props))),
        ("label", preview_page("Label", &label(&label_props))),
        (
            "radio-group",
            preview_page("Radio group", &radio_group(&radio_props, &radio_state)),
        ),
        ("slider", preview_page("Slider", &slider(&slider_props))),
        (
            "switch",
            preview_page("Switch", &switch(&switch_props, &switch_state)),
        ),
        (
            "textarea",
            preview_page("Textarea", &textarea(&textarea_props, &textarea_state)),
        ),
        ("tooltip", preview_page("Tooltip", &tooltip(&tooltip_props))),
        ("spinner", preview_page("Spinner", &spinner(&spinner_props))),
    ];

    for (name, html) in pages {
        fs::write(out.join(format!("{name}.html")), html).expect("write preview");
    }
    println!("previews written to {}", out.display());
}
