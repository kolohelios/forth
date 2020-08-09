use cursive::theme::{Color, ColorStyle};
use cursive::traits::*;
use cursive::views::Canvas;
use cursive::Printer;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

extern crate machine;

mod flower;
mod map;

fn draw(_: &(), p: &Printer) {
    let x_max = p.size.x as u8;

    for x in 0..x_max {
        let style = ColorStyle::new(Color::RgbLowRes(3, 1, 0), Color::RgbLowRes(0, 0, 0));

        p.with_color(style, |printer| {
            printer.print((x, 1), "░");
        });
    }

    let style = ColorStyle::new(Color::RgbLowRes(1, 5, 1), Color::RgbLowRes(0, 0, 0));

    p.with_color(style, |printer| {
        printer.print((4, 0), "|");
    });
}

fn main() {
    let (clock_tx, clock_rx) = mpsc::channel();

    let handle = thread::spawn(move || clock(clock_tx));
    thread::spawn(move || display_clock(clock_rx));

    // Creates the cursive root - required for every application
    let mut siv = cursive::default();
    siv.add_layer(Canvas::new(()).with_draw(draw).fixed_size((10, 2)));
    siv.run();

    handle.join().unwrap();
}

fn clock(tx: mpsc::Sender<String>) {
    let now = Instant::now();
    for i in 1..10 {
        let message = format!("time is {}", now.elapsed().as_millis());
        tx.send(message).unwrap();
        thread::sleep(Duration::from_millis(200));
    }
}

fn display_clock(rx: mpsc::Receiver<String>) {
    for received in rx {
        println!("Got: {}", received);
    }
}
