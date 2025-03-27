use gpui::{App, Menu, MenuItem};
use gpui_component::input::{Copy, Cut, Paste, Redo, Undo};

pub fn init(cx: &mut App) {
    cx.set_menus(vec![
        Menu {
            name: "GPUI App".into(),
            items: vec![],
        },
        Menu {
            name: "Edit".into(),
            items: vec![
                MenuItem::os_action("Undo", Undo, gpui::OsAction::Undo),
                MenuItem::os_action("Redo", Redo, gpui::OsAction::Redo),
                MenuItem::separator(),
                MenuItem::os_action("Cut", Cut, gpui::OsAction::Cut),
                MenuItem::os_action("Copy", Copy, gpui::OsAction::Copy),
                MenuItem::os_action("Paste", Paste, gpui::OsAction::Paste),
            ],
        },
    ]);
}
