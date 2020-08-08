use rand::{rngs::StdRng, Rng, SeedableRng};

pub mod map {
    use super::*;

    #[derive(Clone, Debug, PartialEq)]
    pub enum MapElement {
        Water { height: i8 },
        Land { height: i8 },
        Void,
    }

    #[derive(Debug, PartialEq)]
    pub struct Map {
        pub content: Vec<Vec<MapElement>>,
    }

    impl Map {
        pub fn new(width: usize, height: usize, seed: u64) -> Map {
            let mut content = vec![vec![MapElement::Void; height]; width];
            let mut rng = StdRng::seed_from_u64(seed);

            if width == 0 || height == 0 {
                panic!("There must be a positive value supplied for both width and height")
            }

            for x in 0..width {
                for y in 0..height {
                    let random_value = rng.gen_range(0, 2);
                    match random_value {
                        0 => {
                            content[x][y] = MapElement::Water {
                                height: rng.gen_range(-128, 127),
                            }
                        }
                        1 => {
                            content[x][y] = MapElement::Land {
                                height: rng.gen_range(-128, 127),
                            }
                        }
                        _ => content[x][y] = MapElement::Void,
                    }
                }
            }

            Map { content }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "There must be a positive value supplied for both width and height")]
    fn test_empty_map_creation() {
        assert_eq!(
            map::Map::new(0, 0, 0),
            map::Map {
                content: vec![vec![]]
            }
        );
    }

    #[test]
    fn test_tiny_map() {
        assert_eq!(
            map::Map::new(2, 2, 0),
            map::Map {
                content: vec![
                    vec![
                        map::MapElement::Land { height: -122 },
                        map::MapElement::Water { height: 91 },
                    ],
                    vec![
                        map::MapElement::Water { height: -104 },
                        map::MapElement::Land { height: -68 },
                    ]
                ]
            }
        )
    }

    #[test]
    fn test_larger_map() {
        assert_eq!(
            map::Map::new(4, 4, 0),
            map::Map {
                content: vec![
                    vec![
                        map::MapElement::Land { height: -122 },
                        map::MapElement::Water { height: 91 },
                        map::MapElement::Water { height: -104 },
                        map::MapElement::Land { height: -68 },
                    ],
                    vec![
                        map::MapElement::Water { height: -100 },
                        map::MapElement::Land { height: -65 },
                        map::MapElement::Water { height: -2 },
                        map::MapElement::Land { height: 68 },
                    ],
                    vec![
                        map::MapElement::Land { height: -61 },
                        map::MapElement::Land { height: -106 },
                        map::MapElement::Water { height: -40 },
                        map::MapElement::Land { height: 83 },
                    ],
                    vec![
                        map::MapElement::Land { height: -55 },
                        map::MapElement::Land { height: 17 },
                        map::MapElement::Water { height: -23 },
                        map::MapElement::Water { height: -118 },
                    ]
                ]
            }
        )
    }
}
