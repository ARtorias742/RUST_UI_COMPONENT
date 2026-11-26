//! Ferrum UI component models.
//!
//! Components are renderer-neutral: they expose props, class/style merges, ARIA
//! and keyboard state machines. Web, SSR and native adapters render the model.

pub mod components;
pub mod registry;

pub use components::{
    accordion, alert, avatar, badge, breadcrumb, button, card, checkbox, dialog, empty_state,
    input, kbd, label, menu, progress, radio_group, separator, skeleton, slider, spinner, switch,
    tabs, textarea, toast, tooltip,
};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
