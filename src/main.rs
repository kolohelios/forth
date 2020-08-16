use cursive::view::Position;
use cursive::views::LayerPosition;
use cursive::Cursive;
use rand::Rng;
use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::time::{Duration, Instant};

extern crate machine;

mod flower;
mod flower_display;
mod map;
mod screen_layout;

fn main() {
    let (mut msg_sender, msg_receiver) = channel();

    let mut siv = cursive::default();

    siv = screen_layout::run_screen_event_loop(siv);

    siv.refresh();

    siv.add_global_callback('q', Cursive::quit);

    thread::spawn(move || clock(msg_sender));
    loop {
        siv.step();
        if !siv.is_running() {
            break;
        }

        let mut needs_refresh = false;

        for m in msg_receiver.try_iter() {
            needs_refresh = true;
            let mut rng = rand::thread_rng();

            let x = rng.gen_range(-50, 50);
            let y = rng.gen_range(-15, 15);

            siv.reposition_layer(LayerPosition::FromFront(0), Position::absolute((x, y)));
        }
        if needs_refresh {
            siv.refresh();
        }
    }
}

fn clock(sender: Sender<String>) {
    let now = Instant::now();
    for i in 1..1000 {
        let message = format!("time is {}", now.elapsed().as_millis());
        sender.send(message).unwrap();
        thread::sleep(Duration::from_millis(200));
    }
}
