use cursive::theme::{Color, ColorStyle};
use cursive::traits::*;
use cursive::view::{Position, SizeConstraint};
use cursive::views::{Canvas, FixedLayout, LayerPosition, OnLayoutView, ResizedView};
use cursive::{Cursive, Printer, Rect, Vec2};

#[path = "flower_display.rs"]
mod flower_display;

fn print_flower(p: &Printer, flower: flower_display::DrawnFlower) {
    let x_max = flower[0].len();
    let y_max = flower.len();

    for y in 0..y_max {
        for x in 0..x_max {
            let flower_glyphxel = &flower[y][x];
            p.with_color(
                ColorStyle::from((
                    flower_glyphxel.foreground_color,
                    flower_glyphxel.background_color,
                )),
                |printer| {
                    printer.print((x, y), &flower_glyphxel.character.to_string());
                },
            )
        }
    }
}

fn draw_flower(_: &(), p: &Printer) {
    let x_max = p.size.x as u8;

    let mut flower = flower_display::draw_empty_space(12, 7, flower_display::FlowerColor::WHITE);
    flower = flower_display::draw_pot(flower, Color::RgbLowRes(5, 5, 5));

    print_flower(p, flower);
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
    let top_panel = Canvas::new(()).with_draw(draw_sky).with_name("sky");
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

pub fn draw_rain(_: &(), p: &Printer) {
    let rain_color = ColorStyle::new(Color::RgbLowRes(1, 3, 5), Color::RgbLowRes(0, 2, 5));

    p.with_color(rain_color, |printer| {
        printer.print((0, 0), "|");
    });
}

fn move_top(c: &mut Cursive, x_in: isize, y_in: isize) {
    let s = c.screen_mut();
    let l = LayerPosition::FromFront(0);

    let pos = s.offset().saturating_add((x_in, y_in));
    let p = Position::absolute(pos);

    s.reposition_layer(l, p);
}

pub fn run_screen_event_loop(mut siv: cursive::Cursive) -> cursive::Cursive {
    siv.load_toml(include_str!("../assets/cursive_theme.toml"))
        .unwrap();
    render(&mut siv);

    siv.add_layer(Canvas::new(()).with_draw(draw_rain));
    siv
}
