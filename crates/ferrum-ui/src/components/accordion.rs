use ferrum_ui_core::a11y::{Aria, Role};
use ferrum_ui_core::event::{Key, KeyEvent};
use ferrum_ui_core::ssr::Element;
use ferrum_ui_core::style::{Class, Style};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccordionItem {
    pub id: String,
    pub title: String,
    pub content: String,
    pub disabled: bool,
}

impl AccordionItem {
    pub fn new(
        id: impl Into<String>,
        title: impl Into<String>,
        content: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            content: content.into(),
            disabled: false,
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccordionProps {
    pub items: Vec<AccordionItem>,
    pub multiple: bool,
    pub default_open: Vec<usize>,
    pub class: Class,
    pub style: Style,
}

impl Default for AccordionProps {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            multiple: false,
            default_open: Vec::new(),
            class: Class::new(),
            style: Style::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum AccordionAction {
    None,
    Toggled(usize),
    Focused(usize),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccordionState {
    pub open: Vec<bool>,
    pub focus: usize,
}

impl AccordionState {
    pub fn new(props: &AccordionProps) -> Self {
        let mut open = vec![false; props.items.len()];
        for index in &props.default_open {
            if let Some(slot) = open.get_mut(*index) {
                *slot = true;
            }
        }
        Self { open, focus: 0 }
    }

    pub fn is_open(&self, index: usize) -> bool {
        self.open.get(index).copied().unwrap_or(false)
    }

    pub fn toggle(&mut self, props: &AccordionProps, index: usize) -> bool {
        if index >= self.open.len() || props.items[index].disabled {
            return false;
        }
        let next = !self.open[index];
        if next && !props.multiple {
            self.open.fill(false);
        }
        self.open[index] = next;
        self.focus = index;
        true
    }

    pub fn handle_key(&mut self, props: &AccordionProps, event: &KeyEvent) -> AccordionAction {
        let len = props.items.len();
        if len == 0 {
            return AccordionAction::None;
        }
        if event.key == Key::Enter || event.key == Key::Space {
            return if self.toggle(props, self.focus) {
                AccordionAction::Toggled(self.focus)
            } else {
                AccordionAction::None
            };
        }
        let next = match event.key {
            Key::ArrowDown | Key::ArrowRight => Some((self.focus + 1) % len),
            Key::ArrowUp | Key::ArrowLeft => Some((self.focus + len - 1) % len),
            Key::Home => Some(0),
            Key::End => Some(len - 1),
            _ => None,
        };
        match next {
            Some(index) => {
                self.focus = index;
                AccordionAction::Focused(index)
            }
            None => AccordionAction::None,
        }
    }

    pub fn trigger_aria(&self, props: &AccordionProps, index: usize) -> Aria {
        let item = &props.items[index];
        Aria::new()
            .set("id", format!("accordion-trigger-{}", item.id))
            .set(
                "aria-expanded",
                if self.is_open(index) { "true" } else { "false" },
            )
            .set("aria-controls", format!("accordion-panel-{}", item.id))
            .set("tabindex", if index == self.focus { "0" } else { "-1" })
    }

    pub fn panel_aria(&self, props: &AccordionProps, index: usize) -> Aria {
        let item = &props.items[index];
        Aria::new()
            .role(Role::Region)
            .set("id", format!("accordion-panel-{}", item.id))
            .labelledby(format!("accordion-trigger-{}", item.id))
            .hidden(!self.is_open(index))
    }

    pub fn element(&self, props: &AccordionProps) -> Element {
        let mut root = Element::new("div")
            .class("ferrum-accordion")
            .style(props.style.clone());
        root.class.merge(&props.class);
        for (index, item) in props.items.iter().enumerate() {
            let mut trigger = Element::new("button")
                .class("ferrum-accordion-trigger")
                .attr("type", "button");
            for (name, value) in self.trigger_aria(props, index).to_attrs() {
                trigger = trigger.attr(name.clone(), value.clone());
            }
            if item.disabled {
                trigger = trigger.attr("disabled", "");
            }
            trigger = trigger.text(item.title.clone());
            let mut panel = Element::new("div").class("ferrum-accordion-panel");
            for (name, value) in self.panel_aria(props, index).to_attrs() {
                panel = panel.attr(name.clone(), value.clone());
            }
            if !self.is_open(index) {
                panel = panel.attr("hidden", "");
            } else {
                panel = panel.text(item.content.clone());
            }
            root = root.child(
                Element::new("div")
                    .class("ferrum-accordion-item")
                    .child(
                        Element::new("h3")
                            .class("ferrum-accordion-heading")
                            .child(trigger),
                    )
                    .child(panel),
            );
        }
        root
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn props() -> AccordionProps {
        AccordionProps {
            items: vec![
                AccordionItem::new("a", "A", "Panel A"),
                AccordionItem::new("b", "B", "Panel B"),
            ],
            ..AccordionProps::default()
        }
    }

    #[test]
    fn single_open_closes_previous_panel() {
        let props = props();
        let mut state = AccordionState::new(&props);
        assert!(state.toggle(&props, 0));
        assert!(state.toggle(&props, 1));
        assert!(!state.is_open(0));
        assert!(state.is_open(1));
    }

    #[test]
    fn keyboard_toggles_focused_item() {
        let props = props();
        let mut state = AccordionState::new(&props);
        assert_eq!(
            state.handle_key(&props, &KeyEvent::new(Key::ArrowDown)),
            AccordionAction::Focused(1)
        );
        assert_eq!(
            state.handle_key(&props, &KeyEvent::new(Key::Space)),
            AccordionAction::Toggled(1)
        );
        assert!(state.is_open(1));
    }
}
