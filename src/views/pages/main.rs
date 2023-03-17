use crate::{
    popups::popup,
    utils::{exec_cmd, get_screen_height, get_screen_width},
    views::base,
    views::pages,
};

use cursive::traits::*;
use cursive::{
    align::HAlign,
    view::Scrollable,
    views::{Dialog, DummyView, LinearLayout, SelectView},
};

pub fn menu_handler(cs: &mut cursive::Cursive, item: &str) {
    match item.to_lowercase().as_str() {
        "file manager" => pages::file_manager::main_screen(cs),
        "git" => pages::git::main_screen(cs),
        "quit" => cs.quit(),
        _ => popup::not_implemented(cs),
    }
}

pub fn main_screen(cs: &mut cursive::Cursive) {
    cs.pop_layer();

    // Left section
    let files = std::fs::read_dir(".").unwrap();

    let sv = SelectView::<String>::new()
        .with(|s| {
            s.add_item("[d] .", ".".to_string());
            s.add_item("[d] ..", "..".to_string());

            for file in files {
                let unwrapped = file.unwrap();
                let path = unwrapped.path();
                let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
                match path {
                    p if p.is_dir() => s.add_item(format!("[d] {}", file_name), file_name),
                    p if p.is_file() => s.add_item(format!("[f] {}", file_name), file_name),
                    p if p.is_symlink() => s.add_item(format!("[l] {}", file_name), file_name),
                    _ => s.add_item(format!("[?] {}", file_name), file_name),
                }
            }

            s.sort();
        })
        .on_submit(|cs, file_name| {
            let attr = std::fs::metadata(file_name).unwrap();
            if attr.is_dir() {
                std::env::set_current_dir(file_name).unwrap();
                pages::main::main_screen(cs);
                return;
            }

            popup::new_page_syntax(cs, None, file_name, file_name);
        });

    let left_section_title = exec_cmd("pwd", None).0;
    let left_layout = LinearLayout::vertical().child(sv.scrollable());
    let left_section = Dialog::around(left_layout)
        .title(left_section_title.trim())
        .title_position(HAlign::Center)
        .full_screen();

    // Right section
    let menu_items = vec!["FILE MANAGER", "GIT", "QUIT"];
    let menu = SelectView::new()
        .with_all_str(menu_items)
        .on_submit(menu_handler)
        .with(|s| s.set_inactive_highlight(false))
        .scrollable();

    let buttons = LinearLayout::vertical().child(DummyView).child(menu);

    let right_section_title = "Menu";
    let right_section = Dialog::around(buttons)
        .title(right_section_title)
        .title_position(HAlign::Center)
        .min_size((get_screen_width(cs) / 4, get_screen_height(cs)));

    let main_content = LinearLayout::horizontal()
        .child(left_section)
        .child(DummyView)
        .child(right_section);

    base::wrap_window(cs, Box::new(main_content));
}
