use crate::{
    popups::popup,
    utils::{
        exec_cmd, exec_git, get_screen_height, get_screen_width, switch_next_screen,
        switch_prev_screen,
    },
    views::base,
    views::pages::git::{log, status},
    views::pages::main::main_screen as main_menu,
};

use cursive::{
    align::HAlign,
    traits::{Resizable, Scrollable, With},
    views::{Dialog, DummyView, LinearLayout, SelectView, TextView},
};

pub fn main_screen(cs: &mut cursive::Cursive) {
    switch_next_screen(cs);
    let menus = vec![
        "ADD",
        "REMOVE",
        "RESET",
        "COMMIT",
        "REBASE",
        "STATUS",
        "LOG",
        "CHERRY-PICK",
        "BACK",
    ];

    let mut description = String::new();
    let (out, _, _) = exec_git(&["branch", "--show-current"]);
    description.push_str(&format!("Current branch: {}\n", out));
    let tv_description = TextView::new(&description);

    let mut unstaged = SelectView::<String>::new().with(|s| s.set_inactive_highlight(false));
    let mut staged = SelectView::<String>::new().with(|s| s.set_inactive_highlight(false));
    let unstaged_label = TextView::new("Unstaged").h_align(HAlign::Center);
    let staged_label = TextView::new("Staged").h_align(HAlign::Center);

    let (short_status, _, _) = exec_git(&["status", "--short"]);

    for line in short_status.lines() {
        let mut line_str = line.to_string();
        line_str = line_str[3..].to_string();
        let mut unstaged_item = vec![line_str.as_str()];
        let mut staged_item = vec![line_str.as_str()];

        match line.chars().nth(0).unwrap() {
            ' ' => staged_item.clear(), // Unmodified on local
            'M' => staged_item.push("Modified"),
            'T' => staged_item.push("Type changed"),
            'A' => staged_item.push("Added"),
            'D' => staged_item.push("Deleted"),
            'R' => staged_item.push("Renamed"),
            'C' => staged_item.push("Copied"),
            'U' => staged_item.push("Updated but unmerged"),
            '?' => staged_item.push("Untracked"),
            '!' => staged_item.push("Ignored"),
            _ => (),
        }

        if staged_item.len() > 0 {
            staged.add_item(staged_item.join(" - "), staged_item[0].to_string());
        }

        match line.chars().nth(1).unwrap() {
            ' ' => unstaged_item.clear(), // Unmodified on local
            'M' => unstaged_item.push("Modified"),
            'T' => unstaged_item.push("Type changed"),
            'A' => unstaged_item.push("Added"),
            'D' => unstaged_item.push("Deleted"),
            'R' => unstaged_item.push("Renamed"),
            'C' => unstaged_item.push("Copied"),
            'U' => unstaged_item.push("Updated but unmerged"),
            '?' => unstaged_item.push("Untracked"),
            '!' => unstaged_item.push("Ignored"),
            _ => (),
        }

        if unstaged_item.len() > 0 {
            unstaged.add_item(unstaged_item.join(" - "), unstaged_item[0].to_string());
        }
    }

    unstaged.set_on_submit(|s, file_path: &str| {
        menu_unstaged(s, file_path);
    });

    staged.set_on_submit(|s, file_path: &str| {
        menu_staged(s, file_path);
    });

    let (left_section_title, _, _) = exec_cmd("pwd", None);
    let left_section = Dialog::around(
        LinearLayout::vertical()
            .child(DummyView)
            .child(tv_description)
            .child(DummyView)
            .child(unstaged_label)
            .child(unstaged)
            .child(DummyView)
            .child(staged_label)
            .child(staged),
    )
    .title(left_section_title.trim())
    .full_screen()
    .scrollable();

    // Right section
    let sv = SelectView::new()
        .with_all_str(menus)
        .on_submit(|s, item| match item {
            "STATUS" => status::main_screen(s),
            "LOG" => log::main_screen(s),
            "BACK" => {
                switch_prev_screen(s);
                main_menu(s);
            }
            _ => popup::not_implemented(s),
        })
        .scrollable()
        .scroll_x(true);

    let right_section = Dialog::around(sv)
        .title("Menu")
        .min_size((get_screen_width(cs) / 4, get_screen_height(cs)));

    // Main screen
    let main_content = LinearLayout::horizontal()
        .child(left_section)
        .child(DummyView)
        .child(right_section)
        .full_screen();

    base::wrap_window(cs, Box::new(main_content));
}

pub fn menu_staged(cs: &mut cursive::Cursive, file_path: &str) {
    let menu_items = vec![
        ("RESET", "reset"),
        ("REMOVE", "remove"),
        ("DIFF", "diff"),
        ("LOG", "log"),
        ("BLAME", "blame"),
    ];

    menu(cs, file_path, menu_items);
}

pub fn menu_unstaged(cs: &mut cursive::Cursive, file_path: &str) {
    let menu_items = vec![
        ("OPEN", "open"),
        ("ADD", "add"),
        ("REMOVE", "remove"),
        ("DIFF", "diff"),
        ("LOG", "log"),
        ("BLAME", "blame"),
    ];

    menu(cs, file_path, menu_items);
}

fn handler(cs: &mut cursive::Cursive, item: &(String, String)) {
    match item.0.to_lowercase().as_str() {
        "add" => git_add(cs, item.1.as_str()),
        "reset" => git_reset(cs, item.1.as_str()),
        "diff" => git_diff(cs, item.1.as_str()),
        "log" => git_log(cs, item.1.as_str()),
        "open" => popup::new_page_syntax(
            cs,
            None,
            item.1.as_str(),
            format!("Reading - {}", item.1.as_str()).as_str(),
        ),
        _ => popup::not_implemented(cs),
    }
}

fn menu(cs: &mut cursive::Cursive, file_path: &str, menus: Vec<(&str, &str)>) {
    let mut menu_items = Vec::new();
    for menu in menus {
        menu_items.push((
            menu.0.to_string(),
            (menu.1.to_string(), file_path.to_string()),
        ));
    }

    let single_menu = SelectView::new()
        .with_all(menu_items)
        .on_submit(handler)
        .with(|s| s.set_inactive_highlight(false))
        .scrollable();

    let layout = LinearLayout::vertical()
        .child(DummyView)
        .child(TextView::new(format!("File: {}", file_path)))
        .child(DummyView)
        .child(single_menu);

    let action_dlg = Dialog::around(layout)
        .title("Actions")
        .button("Close", |s| {
            s.pop_layer();
        })
        .h_align(HAlign::Center)
        .fixed_size((get_screen_width(cs) / 2, get_screen_height(cs) / 2));

    cs.add_layer(action_dlg);
}

fn git_add(cs: &mut cursive::Cursive, file_path: &str) {
    cs.pop_layer();
    let (_, err, code) = exec_git(&["add", file_path]);

    if code == 0 {
        popup::alert_back(cs, "Success!");
    } else {
        popup::alert_back(cs, &err);
    }
}

fn git_reset(cs: &mut cursive::Cursive, file_path: &str) {
    cs.pop_layer();
    let (_, err, code) = exec_git(&["reset", file_path]);

    if code == 0 {
        popup::alert_back(cs, "Success!");
    } else {
        popup::alert_back(cs, &err);
    }
}

fn git_diff(cs: &mut cursive::Cursive, file_path: &str) {
    cs.pop_layer();
    let (mut out, err, code) = exec_git(&["diff", file_path]);

    if code == 0 {
        if out.trim().is_empty() {
            out = "No changes".to_string();
        }
        popup::new_page_syntax(
            cs,
            Some(&out),
            file_path,
            format!("Diff - [{}]", file_path).as_str(),
        );
    } else {
        popup::new_page(cs, &err, format!("Diff - [{}]", file_path).as_str());
    }
}

fn git_log(cs: &mut cursive::Cursive, file_path: &str) {
    cs.pop_layer();
    let (out, err, code) = exec_git(&["log", "--follow", file_path]);

    if code == 0 {
        popup::new_page_syntax(
            cs,
            Some(&out),
            file_path,
            format!("Log - [{}]", file_path).as_str(),
        );
    } else {
        popup::new_page(cs, &err, format!("Log - [{}]", file_path).as_str());
    }
}
