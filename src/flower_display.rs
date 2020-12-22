use cursive::theme::{Color, ColorStyle};

#[derive(Debug, PartialEq)]
/// Contains an enumeration of colors we can chose from
pub struct FlowerColor;

impl FlowerColor {
    pub const BLACK: Color = Color::RgbLowRes(0, 0, 0);
    pub const BLUE: Color = Color::RgbLowRes(1, 2, 5);
    pub const WHITE: Color = Color::RgbLowRes(5, 5, 5);
    pub const LIGHT_BROWN: Color = Color::RgbLowRes(3, 1, 0);
}

#[derive(Clone, Debug, PartialEq)]
pub struct FlowerGlyphxel {
    pub foreground_color: Color,
    pub background_color: Color,
    pub character: char,
}

pub type DrawnFlower = Vec<Vec<FlowerGlyphxel>>;

pub fn draw_pot(mut drawn_flower: DrawnFlower, background_color: Color) -> DrawnFlower {
    let x_max = drawn_flower[0].len();
    let y_max = drawn_flower.len();

    for x in 0..x_max {
        let dirt_color = FlowerColor::LIGHT_BROWN;

        let mut character_first_row = '░';
        let mut character_second_row = '░';
        if x == 0 {
            character_first_row = ' ';
            character_second_row = '\\';
        } else if x == 1 {
            character_first_row = '\\';
        } else if x == x_max - 2 {
            character_first_row = '/';
        } else if x == x_max - 1 {
            character_first_row = ' ';
            character_second_row = '/';
        }

        // Second row from the bottom
        drawn_flower[y_max - 2][x] = FlowerGlyphxel {
            foreground_color: dirt_color,
            background_color,
            character: character_second_row,
        };
        // Last row
        drawn_flower[y_max - 1][x] = FlowerGlyphxel {
            foreground_color: dirt_color,
            background_color,
            character: character_first_row,
        };
    }

    drawn_flower
}

pub fn draw_empty_space(width: usize, height: usize, background_color: Color) -> DrawnFlower {
    let flower = vec![
        vec![
            FlowerGlyphxel {
                foreground_color: FlowerColor::WHITE,
                background_color,
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
    let flower = draw_empty_space(7, 2, FlowerColor::WHITE);
    let flower_pot = draw_pot(flower, Color::RgbLowRes(5, 5, 5));

    assert_eq!(
        flower_pot,
        vec![
            vec![
                FlowerGlyphxel {
                    foreground_color: FlowerColor::LIGHT_BROWN,
                    background_color: FlowerColor::WHITE,
                    character: ' '
                },
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
                    character: '/'
                },
                FlowerGlyphxel {
                    foreground_color: FlowerColor::LIGHT_BROWN,
                    background_color: FlowerColor::WHITE,
                    character: ' '
                },
            ],
            vec![
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
            ]
        ]
    )
}

#[test]
fn test_draw_empty_space() {
    let flower = draw_empty_space(2, 3, FlowerColor::WHITE);
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
