use ferrum_ui_core::a11y::{Aria, Role};
use ferrum_ui_core::event::{Key, KeyEvent};
use ferrum_ui_core::ssr::Element;
use ferrum_ui_core::style::{Class, Style};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RadioOption {
    pub id: String,
    pub label: String,
    pub disabled: bool,
}

impl RadioOption {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            disabled: false,
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RadioGroupProps {
    pub name: String,
    pub options: Vec<RadioOption>,
    pub value: Option<String>,
    pub disabled: bool,
    pub class: Class,
    pub style: Style,
}

impl Default for RadioGroupProps {
    fn default() -> Self {
        Self {
            name: "radio".to_owned(),
            options: Vec::new(),
            value: None,
            disabled: false,
            class: Class::new(),
            style: Style::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RadioGroupAction {
    None,
    Changed(Option<String>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RadioGroupState {
    pub value: Option<String>,
    pub focus: usize,
}

impl RadioGroupState {
    pub fn new(props: &RadioGroupProps) -> Self {
        let focus = props
            .value
            .as_ref()
            .and_then(|value| props.options.iter().position(|option| &option.id == value))
            .unwrap_or(0);
        Self {
            value: props.value.clone(),
            focus,
        }
    }

    pub fn select(&mut self, id: &str) -> bool {
        self.value = Some(id.to_owned());
        true
    }

    pub fn handle_key(&mut self, props: &RadioGroupProps, event: &KeyEvent) -> RadioGroupAction {
        if props.disabled || props.options.is_empty() {
            return RadioGroupAction::None;
        }
        let len = props.options.len();
        let next = match event.key {
            Key::ArrowDown | Key::ArrowRight => Some((self.focus + 1) % len),
            Key::ArrowUp | Key::ArrowLeft => Some((self.focus + len - 1) % len),
            Key::Home => Some(0),
            Key::End => Some(len - 1),
            _ => None,
        };
        if let Some(index) = next {
            self.focus = index;
            let id = props.options[index].id.clone();
            self.value = Some(id.clone());
            return RadioGroupAction::Changed(Some(id));
        }
        RadioGroupAction::None
    }

    pub fn aria(&self) -> Aria {
        Aria::new().role(Role::RadioGroup)
    }

    pub fn element(&self, props: &RadioGroupProps) -> Element {
        let mut group = Element::new("div")
            .class("ferrum-radio-group")
            .style(props.style.clone());
        group.class.merge(&props.class);
        for (name, value) in self.aria().to_attrs() {
            group = group.attr(name.clone(), value.clone());
        }
        if !props.name.is_empty() {
            group = group.attr("aria-label", props.name.clone());
        }
        for (index, option) in props.options.iter().enumerate() {
            let mut input = Element::new("input")
                .class("ferrum-radio-input")
                .attr("type", "radio")
                .attr("name", props.name.clone())
                .attr("value", option.id.clone());
            if self.value.as_deref() == Some(option.id.as_str()) {
                input = input.attr("checked", "");
            }
            if option.disabled || props.disabled {
                input = input.attr("disabled", "");
            }
            let label = Element::new("label")
                .class("ferrum-radio")
                .child(input)
                .child(Element::new("span").text(option.label.clone()));
            group = group.child(label);
            let _ = index;
        }
        group
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ferrum_ui_core::event::Key;

    fn props() -> RadioGroupProps {
        RadioGroupProps {
            name: "size".into(),
            options: vec![
                RadioOption::new("sm", "Small"),
                RadioOption::new("md", "Medium"),
            ],
            ..RadioGroupProps::default()
        }
    }

    #[test]
    fn arrow_keys_select_next_option() {
        let props = props();
        let mut state = RadioGroupState::new(&props);
        assert_eq!(
            state.handle_key(&props, &KeyEvent::new(Key::ArrowDown)),
            RadioGroupAction::Changed(Some("md".into()))
        );
        assert_eq!(state.value.as_deref(), Some("md"));
    }

    #[test]
    fn radio_group_has_radiogroup_role() {
        let props = props();
        let state = RadioGroupState::new(&props);
        assert!(state
            .element(&props)
            .render()
            .contains("role=\"radiogroup\""));
    }
}
