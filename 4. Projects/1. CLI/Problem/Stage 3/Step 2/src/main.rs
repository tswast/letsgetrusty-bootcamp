use std::rc::Rc;

mod models;

mod db;

mod ui;

mod io_utils;
use io_utils::*;

mod navigator;

fn main() {
    let db = Rc::new(db::JiraDatabase::new("data/db.json".to_owned()));
    let mut navigator = navigator::Navigator::new(db);

    loop {
        clearscreen::clear().unwrap();

        let current_page = match navigator.get_current_page() {
            Some(page) => page,
            None => {
                break;
            }
        };
        if let Err(error) = current_page.draw_page() {
            println!(
                "Error rendering page: {}\nPress any key to continue...",
                error
            );
            wait_for_key_press();
        };
        let action = match current_page.handle_input(&io_utils::get_user_input()) {
            Ok(action) => action,
            Err(error) => {
                println!(
                    "Error processing input: {}\nPress any key to continue...",
                    error
                );
                wait_for_key_press();
                None
            }
        };
        if let Some(action) = action {
            if let Err(error) = navigator.handle_action(action) {
                println!(
                    "Error handing action: {}\nPress any key to continue...",
                    error
                );
                wait_for_key_press();
            }
        }
    }
}
