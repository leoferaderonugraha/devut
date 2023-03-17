use crate::config::CONFIG;
use syntect::parsing::SyntaxSet;

pub fn pad_str(s: &str, width: usize) -> String {
    return format!("{:^width$}", s, width = width);
}

pub fn exec_git(cmd: &[&str]) -> (String, String, i32) {
    let pipe = std::process::Command::new("git")
        .args(cmd)
        .output()
        .expect("Failed to run git");

    let out = String::from_utf8(pipe.stdout).unwrap();
    let err = String::from_utf8(pipe.stderr).unwrap();
    let code = pipe.status.code().unwrap();

    return (out, err, code);
}

pub fn exec_cmd(cmd: &str, args: Option<&[&str]>) -> (String, String, i32) {
    let pipe = std::process::Command::new(cmd)
        .args(args.unwrap_or(&[]))
        .output()
        .expect("Failed to run command");

    let out = String::from_utf8(pipe.stdout).unwrap();
    let err = String::from_utf8(pipe.stderr).unwrap();
    let code = pipe.status.code().unwrap();

    return (out, err, code);
}

pub fn switch_next_screen(cs: &mut cursive::Cursive) {
    let current = cs.active_screen();

    cs.set_screen(current + 1);
}

pub fn switch_prev_screen(cs: &mut cursive::Cursive) {
    let current = cs.active_screen();

    if current > 0 {
        cs.set_screen(current - 1);
    }
}

pub fn get_screen_width(cs: &mut cursive::Cursive) -> usize {
    let size = cs.screen_size();
    return size.x as usize;
}

pub fn get_screen_height(cs: &mut cursive::Cursive) -> usize {
    let size = cs.screen_size();
    return size.y as usize;
}

pub fn syntax_highlight_file(
    data: Option<&str>,
    file_path: &str,
) -> cursive::utils::span::SpannedString<cursive::theme::Style> {
    let content = std::fs::read_to_string(file_path).unwrap_or("Failed to open file!".to_string());

    let syntax_set: &SyntaxSet = &CONFIG.syntax_set_newline;
    let theme = &CONFIG.theme_set.themes[&CONFIG.theme_name];

    let syntax = syntax_set
        .find_syntax_for_file(file_path)
        .unwrap()
        .unwrap_or_else(|| syntax_set.find_syntax_plain_text());

    let mut highlighter = syntect::easy::HighlightLines::new(syntax, &theme);

    let result = cursive_syntect::parse(data.unwrap_or(&content), &mut highlighter, &syntax_set);

    return result.unwrap();
}
