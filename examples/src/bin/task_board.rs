use ferrum_ui::components::dialog::{DialogController, DialogProps};
use ferrum_ui::components::menu::{MenuItem, MenuProps, MenuState};
use ferrum_ui::components::tabs::{Tab, TabsProps, TabsState};
use ferrum_ui::components::toast::{Toast, ToastQueue};
use ferrum_ui_ssr::{dialog, menu, preview_page, tabs, toast_queue};

fn main() {
    let tabs_props = TabsProps {
        tabs: vec![
            Tab::new("todo", "To do", "Three tasks"),
            Tab::new("doing", "Doing", "One task"),
            Tab::new("done", "Done", "Eight tasks"),
        ],
        ..TabsProps::default()
    };
    let tabs_state = TabsState::new(&tabs_props);

    let menu_props = MenuProps {
        items: vec![
            MenuItem::item("new", "New task").with_shortcut("⌘N"),
            MenuItem::item("assign", "Assign"),
            MenuItem::disabled("delete", "Delete"),
        ],
        open: true,
        ..MenuProps::default()
    };
    let menu_state = MenuState::new(&menu_props);

    let dialog_props = DialogProps {
        title: "New task".to_owned(),
        description: "Add a task to the board.".to_owned(),
        open: true,
        ..DialogProps::default()
    };
    let mut controller = DialogController::new();
    controller.open(
        Some("new-task".to_owned()),
        vec!["cancel".to_owned(), "create".to_owned()],
    );

    let mut queue = ToastQueue::new();
    queue.push(Toast::new("created", "Task created"));

    let body = format!(
        "{}{}{}{}",
        tabs(&tabs_props, &tabs_state),
        menu(&menu_props, &menu_state),
        dialog(&dialog_props, &controller),
        toast_queue(&queue)
    );
    print!("{}", preview_page("Task board", &body));
}
