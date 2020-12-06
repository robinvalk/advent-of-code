use std::env;
use std::fs;

struct Solver {

}

impl Solver {
    fn sum_total_yes_answers(groups: &Vec<Vec<String>>) -> i32 {
        let mut sum: i32 = 0;

        for group in groups {
            let mut yes_answers = Vec::new();

            for person in group {
                for answer in person.chars() {
                    if !yes_answers.contains(&answer) {
                        yes_answers.push(answer);
                    }
                }
            }

            sum += yes_answers.len() as i32;
        }

        sum
    }

    fn sum_total_yes_answers_everyone(groups: &Vec<Vec<String>>) -> i32 {
        let mut sum: i32 = 0;

        for group in groups {
            let mut first_run = true;
            let mut yes_answers: Vec<char> = Vec::new();

            for person in group {
                if first_run {
                    person.chars().for_each(|answer| yes_answers.push(answer));
                    first_run = false;
                } else {
                    yes_answers = yes_answers.iter()
                        .filter(|answer| person.contains(**answer))
                        .map(|answer| *answer)
                        .collect();
                }
            }

            sum += yes_answers.len() as i32;
        }

        sum
    }
}

struct Input {
    filename: String,
}

impl Input {
    fn new(filename: String) -> Input {
        Input { filename }
    }

    fn load(&self) -> Vec<Vec<String>> {
        let contents = self.file_contents();
        let group_chunks: Vec<&str> = contents.split("\n\n").collect();

        group_chunks.iter()
            .map(|chunck| Input::split_group_string(chunck))
            .collect()
    }

    fn split_group_string(group: &str) -> Vec<String> {
        group.lines().map(|line| line.to_string()).collect()
    }

    fn file_contents(&self) -> String {
        println!("Loading contents from file: {}", self.filename);

        fs::read_to_string(&self.filename).expect("Something went wrong loading contents from file")
    }
}

fn main() {
    let input = Input::new(
        env::args().nth(1).unwrap_or("input.txt".to_string())
    );

    let data = input.load();

    println!("Answer one: {}", Solver::sum_total_yes_answers(&data));
    println!("Answer two: {}", Solver::sum_total_yes_answers_everyone(&data));
}
