use crate::popups::popup;
use crate::views::base;
use crate::views::pages;

use cursive::traits::{Resizable, With};
use cursive::views::{Dialog, SelectView};

pub fn main_screen(cs: &mut cursive::Cursive) {
    cs.pop_layer();

    let title = "File Manager";
    let files = std::fs::read_dir(".").unwrap();

    let sv = SelectView::<String>::new()
        .with(|s| {
            for file in files {
                let unwrapped = file.unwrap();
                let path = unwrapped.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                s.add_item(file_name, file_name.to_string());
            }
            s.sort();
        })
        .on_submit(|cs, file_name| {
            popup::alert(cs, file_name);
        });

    let dialog = Dialog::around(sv)
        .title(title)
        .button("Back", pages::main::main_screen)
        .full_screen();

    base::wrap_window(cs, Box::new(dialog));
}
