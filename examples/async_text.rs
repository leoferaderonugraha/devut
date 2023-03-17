use cursive::traits::*;
use cursive::views::{Dialog, TextView};

fn main() {
    let mut siv = cursive::default();
    siv.add_layer(Dialog::around(
        TextView::new("Starting...").with_name("text"),
    ));

    let cb_sink = siv.cb_sink().clone();
    siv.add_global_callback('q', |s| s.quit());

    std::thread::spawn(move || {
        update_text(cb_sink);
    });

    let cb_sink = siv.cb_sink().clone();
    std::thread::spawn(move || {
        test_pop(cb_sink);
    });

    siv.run();
}

fn update_text(cb: cursive::CbSink) {
    let duration = std::time::Duration::from_millis(100);
    let cb_sink = cb;

    for num in 0..25 {
        std::thread::sleep(duration);
        cb_sink
            .send(Box::new(move |s| {
                s.call_on_name("text", |v: &mut TextView| v.set_content(num.to_string()));
            }))
            .unwrap();
    }
    // cb_sink.send(Box::new(|s| s.quit())).unwrap();
}

fn test_pop(cb: cursive::CbSink) {
    let duration = std::time::Duration::from_millis(100);
    let cb_sink = cb;

    for num in 0..25 {
        std::thread::sleep(duration);
        cb_sink
            .send(Box::new(move |s| {
                s.add_layer(Dialog::info(num.to_string()));
                std::thread::sleep(std::time::Duration::from_millis(300));
                s.pop_layer();
            }))
            .unwrap();
    }
    // cb_sink.send(Box::new(|s| s.quit())).unwrap();
}
