use cursive::theme::{Color, ColorStyle};
use machine::*;

machine!(
    #[derive(Clone, Debug, PartialEq)]
    enum Flower {
        Seed,
        Germinating {
            fertilized: bool,
            watered: bool,
            sunshine: u8,
        },
        Growing {
            fertilized: bool,
            watered: bool,
            sunshine: u8,
        },
        // Flowering {
        //     fertilized: bool,
        //     watered: bool,
        //     visited_by_bees: u8,
        //     sunshine: u8,
        // },
        // Pollinated {
        //     sunshine: u8,
        //     produces_seed: bool,
        // },
        // Fruiting {
        //     picked: bool,
        // },
        // Wilting {
        //     watered: bool,
        //     sunshine: u8,
        // },
        // Decomposing {
        //     watered: bool,
        // },
        // Dead,
    }
);

#[derive(Clone, Debug, PartialEq)]
pub struct Advance;
#[derive(Clone, Debug, PartialEq)]
pub struct Fertilize;
#[derive(Clone, Debug, PartialEq)]
pub struct Water;
#[derive(Clone, Debug, PartialEq)]
pub struct Sunshine {
    pub amount: u8,
}

transitions!(Flower, [
    (Seed, Advance) => Germinating,
    (Germinating, Fertilize) => [ Germinating, Growing ],
    (Germinating, Water) => [ Germinating, Growing ],
    (Germinating, Sunshine) => [ Germinating, Growing ],
    (Germinating, Advance) => Growing
]);

impl Seed {
    pub fn on_advance(self, _: Advance) -> Germinating {
        Germinating {
            fertilized: false,
            watered: false,
            sunshine: 0,
        }
    }
}

impl Germinating {
    pub fn on_advance(self, _: Advance) -> Growing {
        Growing {
            fertilized: false,
            watered: false,
            sunshine: 0,
        }
    }

    pub fn on_fertilize(self, _: Fertilize) -> Flower {
        if self.sunshine > 4 && self.watered == true {
            return Flower::growing(false, false, 0);
        }
        Flower::germinating(true, self.watered, self.sunshine)
    }

    pub fn on_water(self, _: Water) -> Flower {
        if self.fertilized && self.sunshine > 4 {
            return Flower::growing(false, false, 0);
        }
        Flower::germinating(self.fertilized, true, self.sunshine)
    }

    pub fn on_sunshine(self, input: Sunshine) -> Flower {
        let new_sunshine = self.sunshine + input.amount;
        if self.fertilized && self.watered && new_sunshine > 4 {
            return Flower::growing(false, false, 0);
        }
        Flower::germinating(self.fertilized, self.watered, new_sunshine)
    }
}

impl Growing {
    pub fn on_advance(self, _: Advance) -> Seed {
        Seed {}
    }
}

// methods!(
//     Flower,
//     [
//         Seed, Planted, Growing => fn can_pollinate(&self) -> ()
// ]
// );

impl Flower {
    pub fn display(&self) -> () {
        match self {
            Flower::Seed(ref _v) => println!("\x1b[1;30m Seed! (*)"),
            Flower::Germinating(ref _v) => println!("Planted! (~)"),
            Flower::Growing(ref _v) => println!("\x1b[1;30m Growing! (~) \x1b[0m"),
            _ => println!("Unknown!"),
        }
    }
}

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

pub mod flower {
    use super::*;
    pub fn create_flower() -> Flower {
        let f = Flower::Seed(Seed {});
        f
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_flower() {
        let mut f = flower::create_flower();
        assert_eq!(f, Flower::seed());
        f = f.on_advance(Advance);
        assert_eq!(f, Flower::germinating(false, false, 0));
        f = f.on_fertilize(Fertilize);
        assert_eq!(f, Flower::germinating(true, false, 0));
        f = f.on_water(Water);
        assert_eq!(f, Flower::germinating(true, true, 0));
        f = f.on_sunshine(Sunshine { amount: 3 });
        assert_eq!(f, Flower::germinating(true, true, 3));
        f = f.on_sunshine(Sunshine { amount: 2 });
        assert_eq!(f, Flower::growing(false, false, 0));
        f.display();
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
    fn test_draw_flower() {
        let flower = draw_empty_space(7, 7);
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
                    FlowerGlyphxel {
                        foreground_color: Color::RgbLowRes(5, 5, 5),
                        background_color: Color::RgbLowRes(0, 0, 0),
                        character: ' '
                    },
                ],
            ]
        );
    }
}
