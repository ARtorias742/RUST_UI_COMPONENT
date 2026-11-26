use ferrum_ui::components::button::{ButtonProps, ButtonSize, ButtonVariant};
use ferrum_ui::components::toast::{Toast, ToastQueue};
use ferrum_ui_ssr::{button, preview_page, toast_queue};

fn main() {
    let primary = ButtonProps::new("New report")
        .with_variant(ButtonVariant::Default)
        .with_size(ButtonSize::Default);
    let secondary = ButtonProps::new("Export")
        .with_variant(ButtonVariant::Outline)
        .with_size(ButtonSize::Sm);
    let mut queue = ToastQueue::new();
    queue.push(
        Toast::new("sync", "Dashboard synced")
            .with_description("Updated 12 widgets")
            .with_duration(3_000),
    );
    let body = format!(
        "<main><h1>Dashboard</h1>{}{}{}</main>",
        button(&primary),
        button(&secondary),
        toast_queue(&queue)
    );
    print!("{}", preview_page("Dashboard", &body));
}
