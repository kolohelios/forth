use cursive::theme::{Color, ColorStyle};
use cursive::traits::*;
use cursive::view::SizeConstraint;
use cursive::views::{Canvas, FixedLayout, OnLayoutView, ResizedView};
use cursive::{Cursive, Printer, Rect, Vec2};

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

fn render(siv: &mut Cursive) {
    let top_panel = Canvas::new(()).with_draw(draw_sky);
    let bottom_panel = Canvas::new(()).with_draw(draw_flower).fixed_size((10, 2));

    let resized_view = ResizedView::new(
        SizeConstraint::Full,
        SizeConstraint::Full,
        // Dummy sizes that get overriden in OnLayoutView
        FixedLayout::new()
            .child(Rect::from_size((0, 0), (1, 1)), top_panel)
            .child(Rect::from_size((0, 0), (1, 1)), bottom_panel),
    );

    let view = OnLayoutView::new(resized_view, |v, s| {
        let fixed_layout = v.get_inner_mut();

        let top_panel_top_left = Vec2::new(0, 0);
        let top_panel_bottom_right = Vec2::new(s.x, (s.y as f32 * 0.4) as usize);
        let top_size = Rect::from_corners(top_panel_top_left, top_panel_bottom_right);
        fixed_layout.set_child_position(0, top_size);

        let bottom_panel_top_left = Vec2::new(0, (s.y as f32 * 0.45) as usize);
        let bottom_panel_bottom_right = Vec2::new(s.x, s.y);
        let bottom_size = Rect::from_corners(bottom_panel_top_left, bottom_panel_bottom_right);
        fixed_layout.set_child_position(1, bottom_size);

        v.layout(s);
    });

    siv.add_fullscreen_layer(view);
}

pub fn run_screen_event_loop() {
    let mut siv = cursive::default();
    render(&mut siv);
    siv.run();
}
