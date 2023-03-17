use crate::utils::exec_git;
use crate::views::pages;

use cursive::align::HAlign;
use cursive::traits::{Resizable, Scrollable};
use cursive::views::{Dialog, Panel, TextView};

pub fn main_screen(cs: &mut cursive::Cursive) {
    cs.pop_layer();

    let title = "Git Log";
    let cmd = vec!["log", "--oneline", "--decorate", "--graph"];
    let (out, _, _) = exec_git(&cmd);

    let tv_output = TextView::new(out).no_wrap().scrollable().scroll_x(true);

    let dialog = Dialog::around(Panel::new(tv_output))
        .title(title)
        .h_align(HAlign::Center)
        .button("Back", |s| {
            s.pop_layer();
            pages::main::main_screen(s);
        })
        .full_screen();

    cs.add_layer(dialog);
}
