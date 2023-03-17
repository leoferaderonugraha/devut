use cursive::traits::*;
use cursive::views::{Dialog, TextView};

fn main() {
    let mut siv = cursive::default();
    siv.add_layer(Dialog::around(
        TextView::new("Starting...").with_name("text"),
    ));

    let cb_sink = siv.cb_sink().clone();

    std::thread::spawn(move || {
        update_time(cb_sink);
    });

    siv.add_global_callback('q', |s| s.quit());

    siv.run();
}

fn update_time(cb: cursive::CbSink) {
    let cb_sink = cb;

    loop {
        cb_sink
            .send(Box::new(move |s| {
                s.call_on_name("text", |v: &mut TextView| {
                    let now = std::process::Command::new("date")
                        .arg("+%Y-%m-%d %H:%M:%S")
                        .output()
                        .expect("failed to execute process");
                    let now = String::from_utf8_lossy(&now.stdout);
                    v.set_content(now.to_string());
                });
                std::thread::sleep(std::time::Duration::from_millis(1000));
            }))
            .unwrap();

        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}
