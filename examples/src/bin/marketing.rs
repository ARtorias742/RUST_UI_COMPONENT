use ferrum_ui::components::button::{ButtonProps, ButtonSize, ButtonVariant};
use ferrum_ui::components::menu::{MenuItem, MenuProps, MenuState};
use ferrum_ui_ssr::{button, menu, preview_page};

fn main() {
    let cta = ButtonProps::new("Start free")
        .with_variant(ButtonVariant::Default)
        .with_size(ButtonSize::Lg);
    let links = MenuProps {
        items: vec![
            MenuItem::item("product", "Product"),
            MenuItem::item("pricing", "Pricing"),
            MenuItem::item("docs", "Docs"),
        ],
        open: true,
        ..MenuProps::default()
    };
    let state = MenuState::new(&links);
    let body = format!(
        "<main><h1>Ship accessible UI</h1><p>Copy the source. Own the component.</p>{}{}</main>",
        button(&cta),
        menu(&links, &state)
    );
    print!("{}", preview_page("Marketing", &body));
}
