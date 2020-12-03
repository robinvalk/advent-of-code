use std::env;
use std::fs;

struct Input {
    filename: String,
}

impl Input {
    fn new(filename: String) -> Input {
        Input { filename }
    }

    fn load_data(&self) -> Vec<u32> {
        return self.file_contents().lines()
            .map(|line| line.parse::<u32>().unwrap())
            .collect();
    }

    fn file_contents(&self) -> String {
        println!("Loading contents from file: {}", self.filename);

        return fs::read_to_string(&self.filename)
            .expect("Something went wrong loading data from file");
    }
}

fn main() {
    let filename = env::args().nth(1).unwrap_or("input.txt".to_string());
    let input_data = Input::new(filename).load_data();

    'answer_one: for a in input_data.iter().copied() {
        for b in input_data.iter().copied() {
            if (a + b) == 2020 {
                println!("Answer one: {}", a*b);
                break 'answer_one;
            }
        }
    }

    'answer_two: for a in input_data.iter().copied() {
        for b in input_data.iter().copied() {
            for c in input_data.iter().copied() {
                if (a + b + c) == 2020 {
                    println!("Answer two: {}", a*b*c);
                    break 'answer_two;
                }
            }
        }
    }
}
