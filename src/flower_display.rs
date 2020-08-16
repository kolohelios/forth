use cursive::theme::{Color, ColorStyle};

#[derive(Debug, PartialEq)]
pub struct FlowerColor;

impl FlowerColor {
    pub const BLACK: Color = Color::RgbLowRes(0, 0, 0);
    pub const BLUE: Color = Color::RgbLowRes(1, 2, 5);
    pub const WHITE: Color = Color::RgbLowRes(5, 5, 5);
    pub const LIGHT_BROWN: Color = Color::RgbLowRes(3, 1, 0);
}

#[derive(Clone, Debug, PartialEq)]
pub struct FlowerGlyphxel {
    foreground_color: Color,
    background_color: Color,
    character: char,
}

type DrawnFlower = Vec<Vec<FlowerGlyphxel>>;

fn draw_pot(mut drawn_flower: DrawnFlower) -> DrawnFlower {
    let x_max = drawn_flower[0].len();
    let y_max = drawn_flower[0].len();

    for x in 0..x_max {
        let dirt_color = FlowerColor::LIGHT_BROWN;
        let background_color = FlowerColor::WHITE;

        let character;
        if x == 0 {
            character = '\\';
        } else if x == x_max - 1 {
            character = '/';
        } else {
            character = '░';
        }

        drawn_flower[0][x] = FlowerGlyphxel {
            foreground_color: dirt_color,
            background_color: background_color,
            character,
        };
    }

    drawn_flower
}

pub fn draw_empty_space(width: usize, height: usize) -> DrawnFlower {
    let flower = vec![
        vec![
            FlowerGlyphxel {
                foreground_color: FlowerColor::WHITE,
                background_color: FlowerColor::BLACK,
                character: ' '
            };
            width
        ];
        height
    ];

    flower
}

#[test]
fn test_draw_flower_pot() {
    let flower = draw_empty_space(7, 1);
    let flower_pot = draw_pot(flower);

    assert_eq!(
        flower_pot,
        vec![vec![
            FlowerGlyphxel {
                foreground_color: FlowerColor::LIGHT_BROWN,
                background_color: FlowerColor::WHITE,
                character: '\\'
            },
            FlowerGlyphxel {
                foreground_color: FlowerColor::LIGHT_BROWN,
                background_color: FlowerColor::WHITE,
                character: '░'
            },
            FlowerGlyphxel {
                foreground_color: FlowerColor::LIGHT_BROWN,
                background_color: FlowerColor::WHITE,
                character: '░'
            },
            FlowerGlyphxel {
                foreground_color: FlowerColor::LIGHT_BROWN,
                background_color: FlowerColor::WHITE,
                character: '░'
            },
            FlowerGlyphxel {
                foreground_color: FlowerColor::LIGHT_BROWN,
                background_color: FlowerColor::WHITE,
                character: '░'
            },
            FlowerGlyphxel {
                foreground_color: FlowerColor::LIGHT_BROWN,
                background_color: FlowerColor::WHITE,
                character: '░'
            },
            FlowerGlyphxel {
                foreground_color: FlowerColor::LIGHT_BROWN,
                background_color: FlowerColor::WHITE,
                character: '/'
            },
        ]]
    )
}

#[test]
fn test_draw_empty_space() {
    let flower = draw_empty_space(2, 3);
    assert_eq!(
        flower,
        vec![
            vec![
                FlowerGlyphxel {
                    foreground_color: Color::RgbLowRes(5, 5, 5),
                    background_color: Color::RgbLowRes(0, 0, 0),
                    character: ' '
                },
                FlowerGlyphxel {
                    foreground_color: Color::RgbLowRes(5, 5, 5),
                    background_color: Color::RgbLowRes(0, 0, 0),
                    character: ' '
                },
            ],
            vec![
                FlowerGlyphxel {
                    foreground_color: Color::RgbLowRes(5, 5, 5),
                    background_color: Color::RgbLowRes(0, 0, 0),
                    character: ' '
                },
                FlowerGlyphxel {
                    foreground_color: Color::RgbLowRes(5, 5, 5),
                    background_color: Color::RgbLowRes(0, 0, 0),
                    character: ' '
                },
            ],
            vec![
                FlowerGlyphxel {
                    foreground_color: Color::RgbLowRes(5, 5, 5),
                    background_color: Color::RgbLowRes(0, 0, 0),
                    character: ' '
                },
                FlowerGlyphxel {
                    foreground_color: Color::RgbLowRes(5, 5, 5),
                    background_color: Color::RgbLowRes(0, 0, 0),
                    character: ' '
                },
            ],
        ]
    );
}
