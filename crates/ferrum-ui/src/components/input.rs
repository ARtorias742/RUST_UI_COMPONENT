use ferrum_ui_core::a11y::{Aria, Role};
use ferrum_ui_core::event::{Key, KeyEvent};
use ferrum_ui_core::ssr::Element;
use ferrum_ui_core::style::{Class, Style};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[non_exhaustive]
pub enum InputKind {
    #[default]
    Text,
    Email,
    Password,
    Search,
}

impl InputKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Email => "email",
            Self::Password => "password",
            Self::Search => "search",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputProps {
    pub label: String,
    pub value: String,
    pub placeholder: String,
    pub kind: InputKind,
    pub disabled: bool,
    pub invalid: bool,
    pub class: Class,
    pub style: Style,
}

impl Default for InputProps {
    fn default() -> Self {
        Self {
            label: "Input".to_owned(),
            value: String::new(),
            placeholder: String::new(),
            kind: InputKind::Text,
            disabled: false,
            invalid: false,
            class: Class::new(),
            style: Style::new(),
        }
    }
}

impl InputProps {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            ..Self::default()
        }
    }

    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self
    }

    pub fn with_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn with_kind(mut self, kind: InputKind) -> Self {
        self.kind = kind;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn invalid(mut self, invalid: bool) -> Self {
        self.invalid = invalid;
        self
    }

    pub fn class(&self) -> Class {
        let mut class = Class::from("ferrum-input");
        if self.invalid {
            class.add("is-invalid");
        }
        class.merge(&self.class);
        class
    }

    pub fn aria(&self) -> Aria {
        let mut aria = Aria::new().role(Role::Textbox);
        if self.invalid {
            aria = aria.set("aria-invalid", "true");
        }
        if self.disabled {
            aria = aria.set("aria-disabled", "true");
        }
        aria
    }

    pub fn element(&self, state: &InputState) -> Element {
        let mut input = Element::new("input")
            .class(self.class())
            .style(self.style.clone())
            .attr("type", self.kind.as_str())
            .attr("value", state.value.clone());
        if !self.placeholder.is_empty() {
            input = input.attr("placeholder", self.placeholder.clone());
        }
        for (name, value) in self.aria().to_attrs() {
            input = input.attr(name.clone(), value.clone());
        }
        if self.disabled {
            input = input.attr("disabled", "");
        }
        Element::new("label")
            .class("ferrum-input-label")
            .child(Element::new("span").text(self.label.clone()))
            .child(input)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum InputAction {
    None,
    Changed(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputState {
    pub value: String,
}

impl InputState {
    pub fn new(props: &InputProps) -> Self {
        Self {
            value: props.value.clone(),
        }
    }

    pub fn set(&mut self, value: impl Into<String>) {
        self.value = value.into();
    }

    pub fn handle_key(&mut self, props: &InputProps, event: &KeyEvent) -> InputAction {
        if props.disabled {
            return InputAction::None;
        }
        match event.key {
            Key::Backspace => {
                self.value.pop();
                InputAction::Changed(self.value.clone())
            }
            Key::Char(value) if event.is_printable() => {
                self.value.push(value);
                InputAction::Changed(self.value.clone())
            }
            Key::Space if event.is_printable() => {
                self.value.push(' ');
                InputAction::Changed(self.value.clone())
            }
            _ => InputAction::None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ferrum_ui_core::event::Key;

    #[test]
    fn typing_and_backspace_update_state() {
        let props = InputProps::new("Email");
        let mut state = InputState::new(&props);
        assert_eq!(
            state.handle_key(&props, &KeyEvent::new(Key::Char('a'))),
            InputAction::Changed("a".into())
        );
        assert_eq!(
            state.handle_key(&props, &KeyEvent::new(Key::Backspace)),
            InputAction::Changed(String::new())
        );
    }

    #[test]
    fn invalid_input_is_exposed_to_at() {
        let props = InputProps::new("Email").invalid(true);
        let state = InputState::new(&props);
        let html = props.element(&state).render();
        assert!(html.contains("aria-invalid=\"true\""));
        assert!(html.contains("type=\"text\""));
    }
}
