//! Ferrum UI website rendered with the Ferrum Rust frontend framework.
//!
//! The component models come from the local `ferrum-ui` crate; this app converts
//! their renderer-neutral `Element` trees into Ferrum `VNode`s.
//!
//! Run with:
//!   rustup target add wasm32-unknown-unknown
//!   cargo install trunk
//!   trunk serve

use ferrum::prelude::*;
use ferrum::AppHandle;
use ferrum_ui_core_local::ssr::{Element, Node};
use ferrum_ui_local::components::accordion::{
    AccordionItem, AccordionProps, AccordionState,
};
use ferrum_ui_local::components::alert::{AlertProps, AlertVariant};
use ferrum_ui_local::components::avatar::{AvatarProps, AvatarSize};
use ferrum_ui_local::components::badge::{BadgeProps, BadgeVariant};
use ferrum_ui_local::components::button::{ButtonProps, ButtonSize, ButtonVariant};
use ferrum_ui_local::components::card::CardProps;
use ferrum_ui_local::components::checkbox::{CheckboxProps, CheckboxState};
use ferrum_ui_local::components::dialog::{DialogController, DialogProps};
use ferrum_ui_local::components::input::{InputKind, InputProps, InputState};
use ferrum_ui_local::components::menu::{MenuItem, MenuProps, MenuState};
use ferrum_ui_local::components::progress::ProgressProps;
use ferrum_ui_local::components::separator::SeparatorProps;
use ferrum_ui_local::components::skeleton::SkeletonProps;
use ferrum_ui_local::components::tabs::{Tab, TabsProps, TabsState};
use ferrum_ui_local::components::toast::{Toast, ToastQueue};
use wasm_bindgen::prelude::*;

struct ComponentDoc {
    name: &'static str,
    category: &'static str,
    description: &'static str,
    code: &'static str,
}

const COMPONENTS: &[ComponentDoc] = &[
    ComponentDoc { name: "Accordion", category: "Layout", description: "Single or multiple open panels with roving focus.", code: "AccordionProps { items, multiple: false }" },
    ComponentDoc { name: "Alert", category: "Feedback", description: "Semantic status message with four variants.", code: "AlertProps::new(\"Saved\").with_variant(Success)" },
    ComponentDoc { name: "Avatar", category: "Data", description: "Accessible initials avatar with size tokens.", code: "AvatarProps::new(\"AB\").with_size(Lg)" },
    ComponentDoc { name: "Badge", category: "Data", description: "Compact status label with six variants.", code: "BadgeProps::new(\"New\").with_variant(Success)" },
    ComponentDoc { name: "Button", category: "Action", description: "Six variants, four sizes, loading and disabled states.", code: "ButtonProps::new(\"Save\").with_variant(Primary)" },
    ComponentDoc { name: "Card", category: "Layout", description: "Titled content container with optional footer.", code: "CardProps::new(\"Project\").with_footer(\"Done\")" },
    ComponentDoc { name: "Checkbox", category: "Form", description: "Checked, disabled and indeterminate states.", code: "CheckboxProps::new(\"Accept\").indeterminate(true)" },
    ComponentDoc { name: "Dialog", category: "Layout", description: "Portal-friendly modal with focus trap and restore.", code: "DialogController::new().open(trigger, focusables)" },
    ComponentDoc { name: "Input", category: "Form", description: "Text, email, password and search inputs with invalid state.", code: "InputProps::new(\"Email\").with_kind(Email)" },
    ComponentDoc { name: "Menu", category: "Action", description: "Roving tabindex, typeahead, Escape and outside click.", code: "MenuState::new(&props).handle_key(&props, &event)" },
    ComponentDoc { name: "Progress", category: "Data", description: "Determinate progress bar with clamped ARIA values.", code: "ProgressProps::new(64).with_label(\"Upload\")" },
    ComponentDoc { name: "Separator", category: "Layout", description: "Horizontal or vertical rule, decorative or semantic.", code: "SeparatorProps::new().vertical()" },
    ComponentDoc { name: "Skeleton", category: "Feedback", description: "Hidden-from-AT loading placeholder with size tokens.", code: "SkeletonProps::new().with_size(\"180px\", \"12px\")" },
    ComponentDoc { name: "Tabs", category: "Layout", description: "Automatic or manual activation with lazy panels.", code: "TabsProps { activation: Manual, lazy: true }" },
    ComponentDoc { name: "Toast", category: "Feedback", description: "Deduplicated queue with polite live region and pause.", code: "ToastQueue::new().push(Toast::new(\"saved\", \"Saved\"))" },
    ComponentDoc { name: "Breadcrumb", category: "Layout", description: "Navigation trail with separators and current-page semantics.", code: "BreadcrumbProps::new(vec![\"Home\".into(), \"Docs\".into()])" },
    ComponentDoc { name: "EmptyState", category: "Layout", description: "Centered placeholder for empty collections and search results.", code: "EmptyStateProps::new(\"No results\")" },
    ComponentDoc { name: "Kbd", category: "Action", description: "Keyboard shortcut badge.", code: "KbdProps::new(\"⌘K\")" },
    ComponentDoc { name: "Label", category: "Form", description: "Form label bound to a control id.", code: "LabelProps::new(\"Email\").with_for(\"email\")" },
    ComponentDoc { name: "RadioGroup", category: "Form", description: "Radio group with arrow-key selection.", code: "RadioGroupProps { name, options, value }" },
    ComponentDoc { name: "Slider", category: "Form", description: "Range slider with clamped value and step.", code: "SliderProps::new(64).with_range(0, 100)" },
    ComponentDoc { name: "Switch", category: "Form", description: "On/off switch with role=switch semantics.", code: "SwitchProps::new(\"Dark mode\").checked(true)" },
    ComponentDoc { name: "Textarea", category: "Form", description: "Multiline input with label, placeholder and invalid state.", code: "TextareaProps::new(\"Bio\").with_rows(4)" },
    ComponentDoc { name: "Tooltip", category: "Action", description: "Short contextual hint with tooltip semantics.", code: "TooltipProps::new(\"More info\").with_label(\"Info\")" },
    ComponentDoc { name: "Spinner", category: "Feedback", description: "Indeterminate loading indicator with status semantics.", code: "SpinnerProps::new().with_size(Lg)" },
];

fn to_vnode(element: &Element) -> VNode {
    let mut rendered = VNode::element(element.tag.clone());
    rendered.attrs = element
        .attrs
        .iter()
        .map(|(name, value)| (name.clone(), AttrValue::Str(value.clone())))
        .collect();
    if !element.class.is_empty() {
        rendered
            .attrs
            .push(("class".to_owned(), AttrValue::Str(element.class.to_string())));
    }
    if !element.style.is_empty() {
        rendered
            .attrs
            .push(("style".to_owned(), AttrValue::Str(element.style.to_css())));
    }
    rendered.children = element
        .children
        .iter()
        .map(|child| match child {
            Node::Element(element) => to_vnode(element),
            Node::Text(text) => VNode::text(text.clone()),
            _ => VNode::Empty,
        })
        .collect();
    VNode::Element(rendered)
}

#[component]
fn Gallery() -> VNode {
    let button = to_vnode(
        &ButtonProps::new("Primary")
            .with_variant(ButtonVariant::Default)
            .with_size(ButtonSize::Default)
            .element(),
    );
    let outline = to_vnode(
        &ButtonProps::new("Outline")
            .with_variant(ButtonVariant::Outline)
            .element(),
    );
    let badge = to_vnode(&BadgeProps::new("v0.1").with_variant(BadgeVariant::Success).element());
    let avatar = to_vnode(
        &AvatarProps::new("AB")
            .with_label("Ada Lovelace")
            .with_size(AvatarSize::Md)
            .element(),
    );

    let checkbox_props = CheckboxProps::new("Accept terms");
    let checkbox_state = CheckboxState::new(&checkbox_props);
    let checkbox = to_vnode(&checkbox_props.element(&checkbox_state));

    let input_props = InputProps::new("Email")
        .with_placeholder("you@example.com")
        .with_kind(InputKind::Email);
    let input_state = InputState::new(&input_props);
    let input = to_vnode(&input_props.element(&input_state));

    let progress = to_vnode(&ProgressProps::new(64).with_label("Upload").element());
    let card = to_vnode(
        &CardProps::new("Project")
            .with_description("Ship it")
            .with_footer("Done")
            .element(),
    );

    let tabs_props = TabsProps {
        tabs: vec![
            Tab::new("board", "Board", "Board panel"),
            Tab::new("activity", "Activity", "Activity panel"),
        ],
        ..TabsProps::default()
    };
    let tabs_state = TabsState::new(&tabs_props);
    let tabs = to_vnode(&tabs_state.element(&tabs_props));

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
    let menu = to_vnode(&menu_state.element(&menu_props));

    let accordion_props = AccordionProps {
        items: vec![
            AccordionItem::new("a", "Is it accessible?", "Yes. Semantic buttons and ARIA state."),
            AccordionItem::new("b", "Is it styled?", "Yes. Token-driven themes."),
        ],
        default_open: vec![0],
        ..AccordionProps::default()
    };
    let accordion_state = AccordionState::new(&accordion_props);
    let accordion = to_vnode(&accordion_state.element(&accordion_props));

    let alert = to_vnode(
        &AlertProps::new("Changes saved")
            .with_description("Synced just now.")
            .with_variant(AlertVariant::Success)
            .element(),
    );

    let mut queue = ToastQueue::new();
    queue.push(Toast::new("saved", "Changes saved").with_description("Synced just now"));
    let toast = to_vnode(&queue.element());

    let skeleton = to_vnode(&SkeletonProps::new().with_size("180px", "12px").element());
    let separator = to_vnode(&SeparatorProps::new().element());

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
    let dialog = dialog_props
        .element(&controller)
        .map(|element| to_vnode(&element))
        .unwrap_or(VNode::Empty);

    rsx! {
        ThemeRoot { theme: Theme::light(),
            div { class: "page",
                header { class: "hero",
                    h1 { "Ferrum UI" }
                    p { "A shadcn/ui-style component gallery rendered in Rust/WASM." }
                    p { class: "muted", "15 components · same models as the Next.js site" }
                }
                section { class: "grid",
                    div { class: "card",
                        h2 { "Actions" }
                        div { class: "row", {button}, {outline}, {badge}, {avatar} }
                    }
                    div { class: "card",
                        h2 { "Forms" }
                        div { class: "stack", {checkbox}, {input} }
                    }
                    div { class: "card",
                        h2 { "Data"
                        }
                        div { class: "stack", {progress}, {card} }
                    }
                    div { class: "card",
                        h2 { "Navigation" }
                        div { class: "stack", {tabs}, {menu}, {accordion} }
                    }
                    div { class: "card",
                        h2 { "Feedback" }
                        div { class: "stack", {alert}, {toast}, {skeleton} }
                    }
                    div { class: "card",
                        h2 { "Layout" }
                        div { class: "stack", {separator}, {dialog} }
                    }
                }
                section { class: "docs",
                    h2 { "Component index" }
                    p { class: "muted", "All 15 components with usage and accessibility notes." }
                    div { class: "doc-list",
                        for item in COMPONENTS {
                            details { class: "doc",
                                summary { {item.name.to_string()}, span { class: "muted", {item.category.to_string()} } }
                                p { {item.description.to_string()} }
                                code { {item.code.to_string()} }
                            }
                        }
                    }
                }
                footer { "Built with the Ferrum Rust frontend framework + local ferrum-ui models." }
            }
        }
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    let document = web_sys::window()
        .expect("no window")
        .document()
        .expect("no document");
    let root = document
        .get_element_by_id("app")
        .expect("#app not found");
    let app = AppHandle::new(root);
    app.render(|| rsx! { Gallery {} });
    app.forget();
}