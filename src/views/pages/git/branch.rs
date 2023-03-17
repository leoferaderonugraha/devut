use crate::popups::popup;
use crate::utils::exec_git;

use cursive::traits::{Resizable, With};
use cursive::views::{Dialog, SelectView};

pub fn main_screen(cs: &mut cursive::Cursive) {
    cs.pop_layer();

    let title = "Git Branches";
    let cmd = vec!["branch", "-l"];
    let (mut out, _, _) = exec_git(&cmd);

    if out.trim().is_empty() {
        out = "No branch(es) found".to_string();
    }

    let select = SelectView::<String>::new()
        .with(|s| {
            for line in out.lines() {
                s.add_item(line, line.to_string());
            }
        })
        .on_submit(switch_branch);

    let dialog = Dialog::around(select)
        .title(title)
        .button("Back", main_screen)
        .full_screen();

    cs.add_layer(dialog);
}

fn switch_branch(s: &mut cursive::Cursive, branch: &str) {
    let mut target_branch = branch.to_string();

    if target_branch.starts_with('*') || target_branch.starts_with(' ') {
        target_branch = target_branch[2..].to_string();
    }

    let cmd = vec!["checkout", target_branch.as_str()];

    popup::alert(s, &cmd.join("|"));

    let (out, _, _) = exec_git(&cmd);

    let dialog = Dialog::text(out)
        .button("Close", |s| {
            s.pop_layer();
            main_screen(s);
        })
        .fixed_size((50, 10));

    s.add_layer(dialog);
}
