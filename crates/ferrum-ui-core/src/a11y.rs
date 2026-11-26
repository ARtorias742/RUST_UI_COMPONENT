//! ARIA attribute builder.

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Role {
    Button,
    Dialog,
    Menu,
    Menuitem,
    Tab,
    Tablist,
    Tabpanel,
    Status,
    Alert,
    Img,
    Checkbox,
    Textbox,
    Progressbar,
    Region,
    RadioGroup,
    Switch,
    Tooltip,
    Listbox,
    Option,
    Separator,
    Presentation,
}

impl Role {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Button => "button",
            Self::Dialog => "dialog",
            Self::Menu => "menu",
            Self::Menuitem => "menuitem",
            Self::Tab => "tab",
            Self::Tablist => "tablist",
            Self::Tabpanel => "tabpanel",
            Self::Status => "status",
            Self::Alert => "alert",
            Self::Img => "img",
            Self::Checkbox => "checkbox",
            Self::Textbox => "textbox",
            Self::Progressbar => "progressbar",
            Self::Region => "region",
            Self::RadioGroup => "radiogroup",
            Self::Switch => "switch",
            Self::Tooltip => "tooltip",
            Self::Listbox => "listbox",
            Self::Option => "option",
            Self::Separator => "separator",
            Self::Presentation => "presentation",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Live {
    Off,
    Polite,
    Assertive,
}

impl Live {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Off => "off",
            Self::Polite => "polite",
            Self::Assertive => "assertive",
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Aria {
    attrs: Vec<(String, String)>,
}

impl Aria {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        let name = name.into();
        let value = value.into();
        if let Some((_, current)) = self.attrs.iter_mut().find(|(key, _)| key == &name) {
            *current = value;
        } else {
            self.attrs.push((name, value));
        }
        self
    }

    pub fn role(self, role: Role) -> Self {
        self.set("role", role.as_str())
    }

    pub fn label(self, label: impl Into<String>) -> Self {
        self.set("aria-label", label)
    }

    pub fn labelledby(self, id: impl Into<String>) -> Self {
        self.set("aria-labelledby", id)
    }

    pub fn describedby(self, id: impl Into<String>) -> Self {
        self.set("aria-describedby", id)
    }

    pub fn modal(self, modal: bool) -> Self {
        self.set("aria-modal", if modal { "true" } else { "false" })
    }

    pub fn live(self, live: Live) -> Self {
        self.set("aria-live", live.as_str())
    }

    pub fn hidden(self, hidden: bool) -> Self {
        self.set("aria-hidden", if hidden { "true" } else { "false" })
    }

    pub fn haspopup(self, haspopup: impl Into<String>) -> Self {
        self.set("aria-haspopup", haspopup)
    }

    pub fn expanded(self, expanded: bool) -> Self {
        self.set("aria-expanded", if expanded { "true" } else { "false" })
    }

    pub fn selected(self, selected: bool) -> Self {
        self.set("aria-selected", if selected { "true" } else { "false" })
    }

    pub fn controls(self, id: impl Into<String>) -> Self {
        self.set("aria-controls", id)
    }

    pub fn to_attrs(&self) -> &[(String, String)] {
        &self.attrs
    }
}

impl fmt::Display for Aria {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let attrs = self
            .attrs
            .iter()
            .map(|(key, value)| format!("{key}=\"{}\"", escape_attr(value)))
            .collect::<Vec<_>>()
            .join(" ");
        f.write_str(&attrs)
    }
}

fn escape_attr(value: &str) -> String {
    value.replace('&', "&amp;").replace('"', "&quot;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn role_and_live_are_serialised() {
        let aria = Aria::new().role(Role::Dialog).label("Settings").modal(true);
        assert_eq!(
            aria.to_string(),
            "role=\"dialog\" aria-label=\"Settings\" aria-modal=\"true\""
        );
        assert_eq!(
            Aria::new().live(Live::Polite).to_string(),
            "aria-live=\"polite\""
        );
    }
}
