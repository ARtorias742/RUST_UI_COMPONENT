use ferrum_ui_core::a11y::{Aria, Role};
use ferrum_ui_core::event::{Key, KeyEvent};
use ferrum_ui_core::roving::Orientation;
use ferrum_ui_core::ssr::Element;
use ferrum_ui_core::style::{Class, Style};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[non_exhaustive]
pub enum TabActivation {
    #[default]
    Automatic,
    Manual,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tab {
    pub id: String,
    pub label: String,
    pub panel: String,
    pub disabled: bool,
}

impl Tab {
    pub fn new(id: impl Into<String>, label: impl Into<String>, panel: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            panel: panel.into(),
            disabled: false,
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TabsProps {
    pub tabs: Vec<Tab>,
    pub orientation: Orientation,
    pub activation: TabActivation,
    pub lazy: bool,
    pub wrap: bool,
    pub active: usize,
    pub class: Class,
    pub style: Style,
}

impl Default for TabsProps {
    fn default() -> Self {
        Self {
            tabs: Vec::new(),
            orientation: Orientation::Horizontal,
            activation: TabActivation::Automatic,
            lazy: false,
            wrap: true,
            active: 0,
            class: Class::new(),
            style: Style::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum TabsAction {
    None,
    Changed(usize),
    Focused(usize),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TabsState {
    pub active: usize,
    pub focus: usize,
    pub loaded: Vec<bool>,
}

impl TabsState {
    pub fn new(props: &TabsProps) -> Self {
        let len = props.tabs.len();
        let active = props.active.min(len.saturating_sub(1));
        let mut loaded = vec![!props.lazy; len];
        if len > 0 {
            loaded[active] = true;
        }
        Self {
            active,
            focus: active,
            loaded,
        }
    }

    pub fn select(&mut self, index: usize) -> bool {
        if index >= self.loaded.len() {
            return false;
        }
        self.active = index;
        self.focus = index;
        self.loaded[index] = true;
        true
    }

    pub fn is_panel_loaded(&self, index: usize) -> bool {
        self.loaded.get(index).copied().unwrap_or(false)
    }

    pub fn handle_key(&mut self, props: &TabsProps, event: &KeyEvent) -> TabsAction {
        let len = props.tabs.len();
        if len == 0 {
            return TabsAction::None;
        }
        let next = match event.key {
            Key::Home => Some(0),
            Key::End => Some(len - 1),
            Key::ArrowRight if props.orientation == Orientation::Horizontal => {
                Some(self.focus.saturating_add(1))
            }
            Key::ArrowLeft if props.orientation == Orientation::Horizontal => {
                Some(self.focus.saturating_sub(1))
            }
            Key::ArrowDown if props.orientation == Orientation::Vertical => {
                Some(self.focus.saturating_add(1))
            }
            Key::ArrowUp if props.orientation == Orientation::Vertical => {
                Some(self.focus.saturating_sub(1))
            }
            _ => None,
        };
        let next = match next {
            Some(index) if props.wrap => index % len,
            Some(index) => index.min(len - 1),
            None => return TabsAction::None,
        };
        self.focus = next;
        if props.activation == TabActivation::Automatic {
            self.select(next);
            TabsAction::Changed(next)
        } else {
            TabsAction::Focused(next)
        }
    }

    pub fn tab_aria(&self, props: &TabsProps, index: usize) -> Aria {
        let tab = &props.tabs[index];
        Aria::new()
            .role(Role::Tab)
            .selected(index == self.active)
            .controls(format!("panel-{}", tab.id))
            .set("tabindex", if index == self.focus { "0" } else { "-1" })
    }

    pub fn panel_aria(&self, props: &TabsProps, index: usize) -> Aria {
        let tab = &props.tabs[index];
        Aria::new()
            .role(Role::Tabpanel)
            .labelledby(format!("tab-{}", tab.id))
            .hidden(index != self.active)
    }

    pub fn element(&self, props: &TabsProps) -> Element {
        let mut root = Element::new("div").class("ferrum-tabs");
        let mut tablist = Element::new("div")
            .class("ferrum-tablist")
            .attr("role", "tablist")
            .attr(
                "aria-orientation",
                match props.orientation {
                    Orientation::Horizontal => "horizontal",
                    Orientation::Vertical => "vertical",
                    _ => "horizontal",
                },
            );
        for (index, tab) in props.tabs.iter().enumerate() {
            let mut element = Element::new("button")
                .class("ferrum-tab")
                .attr("role", "tab")
                .attr("type", "button")
                .attr("id", format!("tab-{}", tab.id))
                .text(tab.label.clone());
            for (name, value) in self.tab_aria(props, index).to_attrs() {
                element = element.attr(name.clone(), value.clone());
            }
            if tab.disabled {
                element = element.attr("disabled", "");
            }
            tablist = tablist.child(element);
        }
        root = root.child(tablist);
        for (index, tab) in props.tabs.iter().enumerate() {
            let mut panel = Element::new("div")
                .class("ferrum-tabpanel")
                .attr("id", format!("panel-{}", tab.id));
            for (name, value) in self.panel_aria(props, index).to_attrs() {
                panel = panel.attr(name.clone(), value.clone());
            }
            if index != self.active {
                panel = panel.attr("hidden", "");
            }
            if self.is_panel_loaded(index) {
                panel = panel.text(tab.panel.clone());
            }
            root = root.child(panel);
        }
        root
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn props() -> TabsProps {
        TabsProps {
            tabs: vec![
                Tab::new("overview", "Overview", "Overview panel"),
                Tab::new("activity", "Activity", "Activity panel"),
            ],
            ..TabsProps::default()
        }
    }

    #[test]
    fn arrow_keys_change_tab_and_load_panel() {
        let mut state = TabsState::new(&props());
        assert_eq!(
            state.handle_key(&props(), &KeyEvent::new(Key::ArrowRight)),
            TabsAction::Changed(1)
        );
        assert!(state.is_panel_loaded(1));
    }

    #[test]
    fn manual_activation_moves_focus_only() {
        let props = TabsProps {
            activation: TabActivation::Manual,
            ..props()
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
    fn lazy_panels_load_on_selection() {
        let props = TabsProps {
            lazy: true,
            ..props()
        };
        let mut state = TabsState::new(&props);
        assert!(!state.is_panel_loaded(1));
        state.select(1);
        assert!(state.is_panel_loaded(1));
    }
}
