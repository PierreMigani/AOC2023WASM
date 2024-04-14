pub struct CubeSet {
    red_cubes: u32,
    green_cubes: u32,
    blue_cubes: u32,
}

impl CubeSet {
    pub fn new() -> CubeSet {
        CubeSet {
            red_cubes: 0,
            green_cubes: 0,
            blue_cubes: 0,
        }
    }

    fn add_cube(&mut self, cube_type: CubeType, cube_count: u32) {
        match cube_type {
            CubeType::Red => self.red_cubes += 1,
            CubeType::Green => self.green_cubes += 1,
            CubeType::Blue => self.blue_cubes += 1,
        }
    }

    pub fn from_str(str: &str) -> CubeSet {
        let mut cube_groups = Vec::new();

        // split the string by "; "
        let splitted = str.trim().split(", ");

        // loop over each group
        for raw_group in splitted {
            cube_groups.push(CubeGroup::from_str(raw_group));
        }

        CubeSet {
            cube_groups,
        }
    }

    pub fn is_possible(
        &self,
        red_limit: u32,
        green_limit: u32,
        blue_limit: u32
    ) -> bool {
        // loop over each handful
        let mut red_count = 0;
        let mut green_count = 0;
        let mut blue_count = 0;

        for handful in &self.cube_groups {
            match handful.cube_type {
                CubeType::Red => {
                    red_count += handful.cube_count;
                    if red_count > red_limit {
                        return false;
                    }
                },
                CubeType::Green => {
                    green_count += handful.cube_count;
                    if green_count > green_limit {
                        return false;
                    }
                },
                CubeType::Blue => {
                    blue_count += handful.cube_count;
                    if blue_count > blue_limit {
                        return false;
                    }
                },
            }
        }

        true
    }
}
