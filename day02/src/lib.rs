use wasm_bindgen::prelude::*;





struct Game {
    id: u32,
    cube_sets: Vec<CubeSet>,
}

impl Game {
    fn from_line(line: &str) -> Game {
        let mut raw_id = String::new();
        
        // first line begins with "Game #:" # being the game id
        let mut line_chars = line.chars();
        let mut current_char = line_chars.next().unwrap();
        while current_char != ':' {
            if current_char.is_numeric() {
                raw_id.push(current_char);
            }
            current_char = line_chars.next().unwrap();
        }

        let id = raw_id.parse::<u32>().unwrap();

        // get all handfuls in the remaining characters, format is "x Red, y Green, z Blue"
        line_chars.next();
        let mut raw_sets = String::new();
        while let Some(c) = line_chars.next() {
            raw_sets.push(c);
        }
        
        // split by "; "
        let splitted_raw_sets = raw_sets.split("; ");
        let mut cube_sets = Vec::new();
        for raw_set in splitted_raw_sets {
            cube_sets.push(CubeSet::from_str(raw_set));
        }

        Game {
            id,
            cube_sets,
        }
    }

    fn is_possible(
        &self,
        red_limit: u32,
        green_limit: u32,
        blue_limit: u32
    ) -> bool {
        // loop over each handful
        for set in &self.cube_sets {
            if !set.is_possible(red_limit, green_limit, blue_limit) {
                return false;
            }
        }
        
        true
    }
}

fn get_games(
    lines: &Vec<&str>,
) -> Vec<Game> {
    let mut games = Vec::new();

    // loop over each line
    for line in lines {
        games.push(Game::from_line(line));
    }

    games
}

#[wasm_bindgen]
pub fn compute_part1(input: &str) -> u32 {
    let lines = input.lines().collect::<Vec<&str>>();
    let games = get_games(&lines);
    let mut id_sum = 0;
    games.iter().for_each(|game| {
        if game.is_possible(12, 13, 14) {
            id_sum += game.id;
        }
    });

    id_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_part1() {
        let input = "
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        ";
        let result = compute_part1(input);

        assert_eq!(result, 8);
    }
}
