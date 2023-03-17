use cursive::traits::Resizable;
use cursive::views::Dialog;

pub fn wrap_window(cs: &mut cursive::Cursive, view: Box<dyn cursive::View>) {
    let mut title = "DevUT - Development Utility Toolkit".to_string();

    title.push_str(format!(" - [{}]", cs.active_screen()).as_str());

    let dialog = Dialog::around(view).title(title).full_screen();
    cs.add_fullscreen_layer(dialog);
}
