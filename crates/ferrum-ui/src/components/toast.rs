use ferrum_ui_core::a11y::{Aria, Live, Role};
use ferrum_ui_core::ssr::Element;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[non_exhaustive]
pub enum ToastVariant {
    #[default]
    Default,
    Success,
    Danger,
}

impl ToastVariant {
    pub fn class(self) -> &'static str {
        match self {
            Self::Default => "ferrum-toast--default",
            Self::Success => "ferrum-toast--success",
            Self::Danger => "ferrum-toast--danger",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Toast {
    pub id: String,
    pub title: String,
    pub description: String,
    pub variant: ToastVariant,
    pub duration_ms: u64,
}

impl Toast {
    pub fn element(&self) -> Element {
        Element::new("div")
            .class(format!("ferrum-toast {}", self.variant.class()))
            .attr("role", "status")
            .child(Element::new("strong").text(self.title.clone()))
            .child(Element::new("p").text(self.description.clone()))
    }

    pub fn new(id: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            description: String::new(),
            variant: ToastVariant::Default,
            duration_ms: 5_000,
        }
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    pub fn with_variant(mut self, variant: ToastVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn with_duration(mut self, duration_ms: u64) -> Self {
        self.duration_ms = duration_ms;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToastQueue {
    pub toasts: Vec<Toast>,
    pub paused: bool,
    pub max: usize,
}

impl Default for ToastQueue {
    fn default() -> Self {
        Self {
            toasts: Vec::new(),
            paused: false,
            max: 5,
        }
    }
}

impl ToastQueue {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_max(mut self, max: usize) -> Self {
        self.max = max.max(1);
        self
    }

    /// Returns `false` when an existing toast with the same id was replaced.
    pub fn push(&mut self, toast: Toast) -> bool {
        if let Some(existing) = self.toasts.iter_mut().find(|item| item.id == toast.id) {
            *existing = toast;
            return false;
        }
        if self.toasts.len() >= self.max.max(1) {
            self.toasts.remove(0);
        }
        self.toasts.push(toast);
        true
    }

    pub fn dismiss(&mut self, id: &str) -> bool {
        let before = self.toasts.len();
        self.toasts.retain(|toast| toast.id != id);
        before != self.toasts.len()
    }

    pub fn pause(&mut self) {
        self.paused = true;
    }

    pub fn resume(&mut self) {
        self.paused = false;
    }

    pub fn is_paused(&self) -> bool {
        self.paused
    }

    pub fn visible(&self) -> &[Toast] {
        &self.toasts
    }

    pub fn len(&self) -> usize {
        self.toasts.len()
    }

    pub fn is_empty(&self) -> bool {
        self.toasts.is_empty()
    }

    pub fn live(&self) -> Live {
        if self.paused {
            Live::Off
        } else {
            Live::Polite
        }
    }

    pub fn aria(&self) -> Aria {
        Aria::new().role(Role::Status).live(self.live())
    }

    pub fn element(&self) -> Element {
        let mut region = Element::new("div").class("ferrum-toast-region");
        for (name, value) in self.aria().to_attrs() {
            region = region.attr(name.clone(), value.clone());
        }
        for item in self.visible() {
            region = region.child(item.element());
        }
        region
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn duplicate_ids_replace_without_growing_queue() {
        let mut queue = ToastQueue::new();
        assert!(queue.push(Toast::new("saved", "Saved")));
        assert!(!queue.push(Toast::new("saved", "Saved again")));
        assert_eq!(queue.len(), 1);
        assert_eq!(queue.visible()[0].title, "Saved again");
    }

    #[test]
    fn pause_switches_live_region_off() {
        let mut queue = ToastQueue::new();
        queue.push(Toast::new("a", "A"));
        assert_eq!(queue.live(), Live::Polite);
        queue.pause();
        assert_eq!(queue.live(), Live::Off);
        assert!(queue.aria().to_string().contains("aria-live=\"off\""));
    }

    #[test]
    fn max_drops_oldest_toast() {
        let mut queue = ToastQueue::new().with_max(2);
        queue.push(Toast::new("a", "A"));
        queue.push(Toast::new("b", "B"));
        queue.push(Toast::new("c", "C"));
        assert_eq!(queue.visible().len(), 2);
        assert_eq!(queue.visible()[0].id, "b");
    }
}
