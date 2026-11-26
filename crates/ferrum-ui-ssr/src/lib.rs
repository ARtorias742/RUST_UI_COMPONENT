//! SSR renderer and static previews.
//!
//! The renderer is intentionally small: it emits semantic HTML plus the same
//! token variables used by the web adapter, so Playwright can screenshot the
//! exact component model without a JavaScript runtime.

use ferrum_ui::components::accordion::{AccordionProps, AccordionState};
use ferrum_ui::components::alert::AlertProps;
use ferrum_ui::components::avatar::AvatarProps;
use ferrum_ui::components::badge::BadgeProps;
use ferrum_ui::components::breadcrumb::BreadcrumbProps;
use ferrum_ui::components::button::ButtonProps;
use ferrum_ui::components::card::CardProps;
use ferrum_ui::components::checkbox::{CheckboxProps, CheckboxState};
use ferrum_ui::components::dialog::{DialogController, DialogProps};
use ferrum_ui::components::empty_state::EmptyStateProps;
use ferrum_ui::components::input::{InputProps, InputState};
use ferrum_ui::components::kbd::KbdProps;
use ferrum_ui::components::label::LabelProps;
use ferrum_ui::components::menu::{MenuProps, MenuState};
use ferrum_ui::components::progress::ProgressProps;
use ferrum_ui::components::radio_group::{RadioGroupProps, RadioGroupState};
use ferrum_ui::components::separator::SeparatorProps;
use ferrum_ui::components::skeleton::SkeletonProps;
use ferrum_ui::components::slider::SliderProps;
use ferrum_ui::components::spinner::SpinnerProps;
use ferrum_ui::components::switch::{SwitchProps, SwitchState};
use ferrum_ui::components::tabs::{TabsProps, TabsState};
use ferrum_ui::components::textarea::{TextareaProps, TextareaState};
use ferrum_ui::components::toast::{Toast, ToastQueue};
use ferrum_ui::components::tooltip::TooltipProps;
use ferrum_ui_core::ssr::Element;
use ferrum_ui_core::tokens::Theme;

pub fn button(props: &ButtonProps) -> String {
    props.element().render()
}

pub fn accordion(props: &AccordionProps, state: &AccordionState) -> String {
    state.element(props).render()
}

pub fn alert(props: &AlertProps) -> String {
    props.element().render()
}

pub fn avatar(props: &AvatarProps) -> String {
    props.element().render()
}

pub fn badge(props: &BadgeProps) -> String {
    props.element().render()
}

pub fn breadcrumb(props: &BreadcrumbProps) -> String {
    props.element().render()
}

pub fn empty_state(props: &EmptyStateProps) -> String {
    props.element().render()
}

pub fn kbd(props: &KbdProps) -> String {
    props.element().render()
}

pub fn label(props: &LabelProps) -> String {
    props.element().render()
}

pub fn radio_group(props: &RadioGroupProps, state: &RadioGroupState) -> String {
    state.element(props).render()
}

pub fn slider(props: &SliderProps) -> String {
    props.element().render()
}

pub fn spinner(props: &SpinnerProps) -> String {
    props.element().render()
}

pub fn switch(props: &SwitchProps, state: &SwitchState) -> String {
    props.element(state).render()
}

pub fn textarea(props: &TextareaProps, state: &TextareaState) -> String {
    props.element(state).render()
}

pub fn tooltip(props: &TooltipProps) -> String {
    props.element().render()
}

pub fn card(props: &CardProps) -> String {
    props.element().render()
}

pub fn checkbox(props: &CheckboxProps, state: &CheckboxState) -> String {
    props.element(state).render()
}

pub fn input(props: &InputProps, state: &InputState) -> String {
    props.element(state).render()
}

pub fn progress(props: &ProgressProps) -> String {
    props.element().render()
}

pub fn separator(props: &SeparatorProps) -> String {
    props.element().render()
}

pub fn skeleton(props: &SkeletonProps) -> String {
    props.element().render()
}

pub fn dialog(props: &DialogProps, controller: &DialogController) -> String {
    props
        .element(controller)
        .map(|element| element.render())
        .unwrap_or_default()
}

pub fn menu(props: &MenuProps, state: &MenuState) -> String {
    if !state.open {
        return String::new();
    }
    state.element(props).render()
}

pub fn tabs(props: &TabsProps, state: &TabsState) -> String {
    if props.tabs.is_empty() {
        return String::new();
    }
    state.element(props).render()
}

pub fn toast_element(toast: &Toast) -> Element {
    toast.element()
}

pub fn toast(toast: &Toast) -> String {
    toast_element(toast).render()
}

pub fn toast_queue(queue: &ToastQueue) -> String {
    queue.element().render()
}

pub fn preview_page(title: &str, body: &str) -> String {
    let theme = Theme::LIGHT;
    let mut html = String::new();
    html.push_str("<!doctype html><html lang=\"en\" data-theme=\"light\" dir=\"ltr\"><head>");
    html.push_str("<meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">");
    html.push_str(&format!(
        "<title>{}</title>",
        ferrum_ui_core::ssr::escape_html(title)
    ));
    html.push_str("<style>");
    html.push_str(&theme.css_vars());
    html.push_str(":root{color-scheme:light dark}body{margin:0;font-family:ui-sans-serif,system-ui,sans-serif;background:hsl(var(--bg));color:hsl(var(--fg))}");
    html.push_str(".ferrum-dialog-backdrop{display:grid;place-items:center;min-height:100vh}.ferrum-dialog{padding:1.5rem;border-radius:var(--radius)}.ferrum-menu{display:inline-grid;gap:2px;padding:4px}.ferrum-menu-item{text-align:left}.ferrum-tablist{display:flex;gap:4px}.ferrum-tab[aria-selected=true]{font-weight:600}.ferrum-toast-region{display:grid;gap:8px}");
    html.push_str(".ferrum-accordion{display:grid;gap:4px}.ferrum-accordion-trigger{width:100%;text-align:left}.ferrum-accordion-panel{padding:8px}.ferrum-alert{padding:12px;border:1px solid hsl(var(--border));border-radius:var(--radius)}.ferrum-avatar{display:inline-grid;place-items:center;border-radius:999px;background:hsl(var(--muted));color:hsl(var(--muted-fg))}.ferrum-avatar--sm{width:24px;height:24px}.ferrum-avatar--md{width:32px;height:32px}.ferrum-avatar--lg{width:40px;height:40px}.ferrum-badge{display:inline-block;padding:2px 8px;border-radius:999px;border:1px solid hsl(var(--border))}.ferrum-card{border:1px solid hsl(var(--border));border-radius:var(--radius);padding:16px}.ferrum-card-header h2{margin:0}.ferrum-checkbox,.ferrum-input-label{display:grid;gap:4px}.ferrum-input{padding:6px 8px;border:1px solid hsl(var(--border));border-radius:var(--radius)}.ferrum-input.is-invalid{border-color:hsl(var(--danger))}.ferrum-progress{height:8px;background:hsl(var(--muted));border-radius:999px;overflow:hidden}.ferrum-progress-bar{height:100%;background:hsl(var(--ring))}.ferrum-separator{background:hsl(var(--border))}.ferrum-separator--horizontal{height:1px;width:100%}.ferrum-separator--vertical{width:1px;height:1rem}.ferrum-skeleton{background:hsl(var(--muted));border-radius:var(--radius)}");
    html.push_str(".ferrum-breadcrumb-list{display:flex;gap:6px;list-style:none;padding:0}.ferrum-empty-state{padding:24px;text-align:center}.ferrum-kbd{border:1px solid hsl(var(--border));border-radius:4px;padding:1px 4px}.ferrum-label{font-size:.8rem}.ferrum-radio-group{display:grid;gap:4px}.ferrum-radio{display:flex;gap:6px;align-items:center}.ferrum-slider{display:grid;gap:4px}.ferrum-switch{display:inline-flex;gap:6px;align-items:center}.ferrum-switch-track{width:28px;height:16px;border-radius:999px;background:hsl(var(--muted))}.ferrum-textarea{padding:6px 8px;border:1px solid hsl(var(--border));border-radius:var(--radius)}.ferrum-tooltip{border-bottom:1px dotted hsl(var(--muted))}.ferrum-spinner-icon{display:inline-block;width:16px;height:16px;border:2px solid hsl(var(--border));border-top-color:hsl(var(--ring));border-radius:50%;animation:ferrum-spin .8s linear infinite}@keyframes ferrum-spin{to{transform:rotate(360deg)}}");
    html.push_str("@media (prefers-color-scheme: dark){");
    html.push_str(&Theme::DARK.css_vars());
    html.push('}');
    html.push_str("</style></head><body>");
    html.push_str(body);
    html.push_str("</body></html>");
    html
}
