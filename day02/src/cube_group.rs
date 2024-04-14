pub struct CubeGroup {
    cube_type: CubeType,
    cube_count: u32,
}

impl CubeGroup {
    pub fn from_str(str: &str) -> CubeGroup {
        // split the string by " "
        let splitted = str.split(" ");

        let mut raw_count_chars = splitted.clone().next().unwrap().chars();
        // 1: get the cube count
        let mut raw_count = String::new();
        while let Some(c) = raw_count_chars.next() {
            if c.is_numeric() {
                raw_count.push(c);
            }
        }
        let cube_count = raw_count.parse::<u32>().unwrap();


        // 2: get the cube type
        let mut raw_type_chars = splitted.clone().nth(1).unwrap().chars();
        let mut raw_cube_type = String::new();
        while let Some(c) = raw_type_chars.next()  {
            // if is a letter add it to the string
            if c.is_alphabetic() {
                raw_cube_type.push(c);
            }
        }

        let cube_type = match raw_cube_type.as_str() {
            "red" => CubeType::Red,
            "green" => CubeType::Green,
            "blue" => CubeType::Blue,
            _ => panic!("Unknown cube type"),
        };

        CubeGroup {
            cube_type,
            cube_count,
        }
    }
}
