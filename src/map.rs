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
                        0 => content[x as usize][y as usize] = MapElement::Water { height: 0 },
                        1 => content[x as usize][y as usize] = MapElement::Land { height: 0 },
                        _ => content[x as usize][y as usize] = MapElement::Void,
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
                        map::MapElement::Land { height: 0 },
                        map::MapElement::Water { height: 0 },
                    ],
                    vec![
                        map::MapElement::Water { height: 0 },
                        map::MapElement::Water { height: 0 },
                    ]
                ]
            }
        )
    }
}
