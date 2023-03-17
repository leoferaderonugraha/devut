use crate::utils::exec_git;
use crate::views::pages;

use cursive::align::HAlign;
use cursive::traits::{Resizable, Scrollable};
use cursive::views::{Dialog, Panel, TextView};

pub fn main_screen(cs: &mut cursive::Cursive) {
    cs.pop_layer();

    let title = "Git Status";
    let cmd = vec!["status"];
    let (out, _, _) = exec_git(&cmd);

    let text_view = TextView::new(&out).scrollable();

    let dialog = Dialog::around(Panel::new(text_view))
        .title(title)
        .h_align(HAlign::Center)
        .button("Back", |s| {
            s.pop_layer();
            pages::main::main_screen(s);
        })
        .full_screen();

    cs.add_layer(dialog);
}
