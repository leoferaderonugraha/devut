use cursive::event;
use cursive::menu;
use cursive::traits::With;
use cursive::views::Dialog;

fn main() {
    let mut siv = cursive::default();

    siv.menubar()
        .add_subtree(
            "File",
            menu::Tree::new()
                .leaf("New", |s| s.add_layer(Dialog::info("New file!")))
                .subtree(
                    "Recent",
                    menu::Tree::new().with(|tree| {
                        for i in 1..100 {
                            tree.add_leaf(format!("Item {}", i), |_| ())
                        }
                    }),
                )
                .delimiter()
                .with(|tree| {
                    for i in 1..10 {
                        tree.add_leaf(format!("Option {}", i), |_| ());
                    }
                })
                .delimiter()
                .leaf("Quit", |s| s.quit()),
        )
        .add_subtree(
            "Help",
            menu::Tree::new()
                .subtree(
                    "Help",
                    menu::Tree::new()
                        .leaf("General", |s| s.add_layer(Dialog::info("Help message!")))
                        .leaf("Online", |s| s.add_layer(Dialog::info("Online help?"))),
                )
                .leaf("About", |s| s.add_layer(Dialog::info("Cursive v0.0.0"))),
        );

    siv.add_global_callback(event::Key::Esc, |s| s.select_menubar());
    siv.add_global_callback('q', |s| s.quit());

    siv.run();
}
