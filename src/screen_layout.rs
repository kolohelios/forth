use cursive::theme::{Color, ColorStyle};
use cursive::traits::*;
use cursive::views::{Canvas, LinearLayout};
use cursive::Printer;

fn draw_flower(_: &(), p: &Printer) {
    let x_max = p.size.x as u8;

    for x in 0..x_max {
        let dirt_style = ColorStyle::new(Color::RgbLowRes(3, 1, 0), Color::RgbLowRes(0, 0, 0));
        let background_style = ColorStyle::new(Color::RgbLowRes(0, 0, 0), Color::TerminalDefault);

        p.with_color(dirt_style, |printer| {
            printer.print((x, 1), "░");
        });
        p.with_color(background_style, |printer| {
            printer.print((x, 0), " ");
        });
    }

    let style = ColorStyle::new(Color::RgbLowRes(1, 5, 1), Color::TerminalDefault);

    p.with_color(style, |printer| {
        printer.print((4, 0), "|");
    });
}

fn draw_sky(_: &(), p: &Printer) {
    let x_max = p.size.x as u8;
    let y_max = p.size.y as u8;
    let sky_color = ColorStyle::new(Color::RgbLowRes(5, 5, 5), Color::RgbLowRes(0, 2, 5));

    for x in 0..x_max {
        for y in 0..y_max {
            p.with_color(sky_color, |printer| {
                printer.print((x, y), " ");
            });
        }
    }
}

pub fn run_screen_event_loop() {
    let mut siv = cursive::default();
    let linear_layout = LinearLayout::vertical()
        .child(Canvas::new(()).with_draw(draw_sky).fixed_size((100, 10)))
        .child(
            LinearLayout::horizontal()
                .child(Canvas::new(()).with_draw(draw_flower).fixed_size((10, 2))),
        );

    siv.add_fullscreen_layer(linear_layout);

    siv.run();
}
