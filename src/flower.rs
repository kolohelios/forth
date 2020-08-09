use machine::*;

machine!(
    #[derive(Clone, Debug, PartialEq)]
    enum Flower {
        Seed,
        Planted {
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
    (Seed, Advance) => Planted,
    (Planted, Fertilize) => [ Planted, Growing ],
    (Planted, Water) => [ Planted, Growing ],
    (Planted, Sunshine) => [ Planted, Growing ],
    (Planted, Advance) => Growing
]);

impl Seed {
    pub fn on_advance(self, _: Advance) -> Planted {
        Planted {
            fertilized: false,
            watered: false,
            sunshine: 0,
        }
    }
}

impl Planted {
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
        Flower::planted(true, self.watered, self.sunshine)
    }

    pub fn on_water(self, _: Water) -> Flower {
        if self.fertilized && self.sunshine > 4 {
            return Flower::growing(false, false, 0);
        }
        Flower::planted(self.fertilized, true, self.sunshine)
    }

    pub fn on_sunshine(self, input: Sunshine) -> Flower {
        let new_sunshine = self.sunshine + input.amount;
        if self.fertilized && self.watered && new_sunshine > 4 {
            return Flower::growing(false, false, 0);
        }
        Flower::planted(self.fertilized, self.watered, new_sunshine)
    }
}

impl Growing {
    pub fn on_advance(self, _: Advance) -> Seed {
        Seed {}
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
        assert_eq!(f, Flower::planted(false, false, 0));
        f = f.on_fertilize(Fertilize);
        assert_eq!(f, Flower::planted(true, false, 0));
        f = f.on_water(Water);
        assert_eq!(f, Flower::planted(true, true, 0));
        f = f.on_sunshine(Sunshine { amount: 3 });
        assert_eq!(f, Flower::planted(true, true, 3));
        f = f.on_sunshine(Sunshine { amount: 2 });
        assert_eq!(f, Flower::growing(false, false, 0));
    }
}
