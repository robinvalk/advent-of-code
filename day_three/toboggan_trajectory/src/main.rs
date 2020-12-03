use std::fs;
use std::env;

struct TreeHitsChallenge {
    x_movement: usize,
    y_movement: usize,
    lines: Vec<String>,
}

struct Position {
    x: usize,
    y: usize,
}

trait Solver {
    fn answer(&self) -> u64;
}

impl Solver for TreeHitsChallenge {
    fn answer(&self) -> u64 {
        let mut trees_hit = 0;
        let mut current_pos = Position { x: 0, y: 0 };

        while self.lines.len()-1 > current_pos.y {
            let next_y_coord = current_pos.y + self.y_movement;
            let mut next_x_coord = current_pos.x + self.x_movement;
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

struct ManyTreeHitsChallenges {
    challenges: Vec<TreeHitsChallenge>,
}

impl Solver for ManyTreeHitsChallenges {
    fn answer(&self) -> u64 {
        let mut answer: u64 = 1;
        let answers_list = self.challenges.iter().map(|challenge| challenge.answer()).collect::<Vec<u64>>();

        for part in answers_list.iter() {
            answer = part * answer;
        }

        answer
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


        TreeHitsChallenge {x_movement: 3, y_movement: 1, lines}
    }

    fn load_as_many_trees_hits_challenge(&self) -> ManyTreeHitsChallenges {
        let contents = self.file_contents();
        let lines: Vec<String> = contents.lines().map(|line|line.to_string()).collect();

        let mut challenges = Vec::new();
        challenges.push(TreeHitsChallenge {x_movement: 1, y_movement: 1, lines: lines.clone()});
        challenges.push(TreeHitsChallenge {x_movement: 3, y_movement: 1, lines: lines.clone()});
        challenges.push(TreeHitsChallenge {x_movement: 5, y_movement: 1, lines: lines.clone()});
        challenges.push(TreeHitsChallenge {x_movement: 7, y_movement: 1, lines: lines.clone()});
        challenges.push(TreeHitsChallenge {x_movement: 1, y_movement: 2, lines: lines.clone()});

        ManyTreeHitsChallenges { challenges }
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

    let many_trees_hits_challenge = input.load_as_many_trees_hits_challenge();
    println!("Answer two: {}", many_trees_hits_challenge.answer());
}
