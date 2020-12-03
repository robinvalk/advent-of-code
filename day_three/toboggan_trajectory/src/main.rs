use std::fs;
use std::env;

struct TreeHitsChallenge {
    lines: Vec<String>,
}

struct Position {
    x: usize,
    y: usize,
}

impl TreeHitsChallenge {
    fn answer(&self) -> i32 {
        let mut trees_hit = 0;
        let mut current_pos = Position { x: 0, y: 0 };

        while self.lines.len()-1 > current_pos.y {
            let next_y_coord = current_pos.y + 1;
            let mut next_x_coord = current_pos.x + 3;
            next_x_coord = next_x_coord % 31;
            let next_line: &str = self.lines.get(next_y_coord).expect("No next line found anymore!");
            let is_tree = next_line.chars().nth(next_x_coord).expect("X coord out of bounds!");

            if is_tree == '#' {
                trees_hit += 1;
            }

            current_pos.x = next_x_coord;
            current_pos.y = next_y_coord;
        }

        trees_hit
    }
}

struct Input {
    filename: String,
}

impl Input {
    fn new(filename: String) -> Input {
        Input { filename }
    }

    fn load_as_tree_hits_challenge(&self) -> TreeHitsChallenge {
        let contents = self.file_contents();
        let lines: Vec<String> = contents.lines().map(|line|line.to_string()).collect();


        TreeHitsChallenge {lines}
    }

    fn file_contents(&self) -> String {
        println!("Loading contents from file: {}", self.filename);

        return fs::read_to_string(&self.filename).expect("Something went wrong loading contents from file");
    }
}

fn main() {
    let input = Input::new(
        env::args().nth(1).unwrap_or("input.txt".to_string())
    );

    let tree_hits_challenge = input.load_as_tree_hits_challenge();
    println!("Answer one: {}", tree_hits_challenge.answer());
}
