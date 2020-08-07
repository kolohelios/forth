pub mod map {
    pub enum MapElement {
        Water { height: i8 },
        Land { height: i8 },
        Void,
    }

    pub struct Map {
        pub content: Vec<Vec<MapElement>>,
    }

    impl Map {
        pub fn new(width: i8, height: i8) -> Map {
            let mut content = vec![vec![]];

            for x in 0..width - 1 {
                for y in 0..height - 1 {
                    content[x as usize][y as usize] = MapElement::Water { height: 0 }
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
    fn test_empty_map_creation() {
        assert_eq!(
            map::Map::new(0, 0),
            map::Map {
                content: vec![vec![]]
            }
        );
    }

    fn test_small_water_world() {
        assert_eq!(
            map::Map::new(1, 1),
            map::Map {
                content: vec![vec![map::MapElement::Water { height: 0 }]]
            }
        )
    }
}
