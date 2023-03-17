use once_cell::sync::Lazy;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;

pub struct Config {
    pub theme_set: ThemeSet,
    pub theme_name: String,
    pub syntax_set_newline: SyntaxSet,
    pub syntax_set: SyntaxSet,
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    let theme_set = ThemeSet::load_defaults();
    let theme_name = "base16-eighties.dark".to_string();
    let syntax_set_newline = SyntaxSet::load_defaults_newlines();
    let syntax_set = SyntaxSet::load_defaults_nonewlines();

    Config {
        theme_set,
        theme_name,
        syntax_set_newline,
        syntax_set,
    }
});
