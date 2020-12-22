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
        Flowering {
            watered: bool,
            visited_by_bees: u8,
            sunshine: u8,
        },
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
    (Germinating, Advance) => Growing,
    (Growing, Fertilize) => [ Growing, Flowering ],
    (Growing, Water) => [ Growing, Flowering ],
    (Growing, Sunshine) => [ Growing, Flowering ]
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
    pub fn on_advance(self, _: Advance) -> Flower {
        if (self.sunshine > 4 && self.watered && self.fertilized) {
            return Flower::flowering(false, 0, 0);
        }
        Flower::growing(self.fertilized, self.watered, self.sunshine)
    }

    pub fn on_fertilize(self, _: Fertilize) -> Flower {
        if self.sunshine > 4 && self.watered == true {
            return Flower::flowering(false, 0, 0);
        }
        Flower::growing(true, self.watered, self.sunshine)
    }

    pub fn on_water(self, _: Water) -> Flower {
        if self.fertilized && self.sunshine > 4 {
            return Flower::flowering(false, 0, 0);
        }
        Flower::growing(self.fertilized, true, self.sunshine)
    }

    pub fn on_sunshine(self, input: Sunshine) -> Flower {
        let new_sunshine = self.sunshine + input.amount;
        if self.fertilized && self.watered && new_sunshine > 4 {
            return Flower::flowering(false, 0, 0);
        }
        Flower::growing(self.fertilized, self.watered, new_sunshine)
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
        f = f.on_fertilize(Fertilize);
        f = f.on_water(Water);
        f = f.on_sunshine(Sunshine { amount: 10 });
        assert_eq!(f, Flower::flowering(false, 0, 0));
    }
}
