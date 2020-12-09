use std::fs;
use std::env;

struct XMASCypherWeaknessFinder {
    preamble_range: i32,
    numbers: Vec<i64>,
}

impl XMASCypherWeaknessFinder {
    fn find(&mut self) -> i64 {
        for (index, number) in self.numbers.iter().enumerate() {
            if index < self.preamble_range as usize {
                continue;
            }

            let start_index = index-self.preamble_range as usize;
            let found_sum = self.numbers[start_index .. index].iter()
                .filter(|a| {
                    self.numbers[start_index .. index].iter()
                        .filter(|b| {
                            if a == b {
                                return false;
                            }

                            (**a + **b) == *number
                        })
                        .count() > 0
                })
                .count() > 0;

            if !found_sum {
                return *number;
            }
        }

        -1
    }
}

struct XMASEncryptionCracker {
    weak_number: i64,
    numbers: Vec<i64>,
}

impl XMASEncryptionCracker {
    fn crack(&mut self) -> i64 {
        for (start_index, _number) in self.numbers.iter().enumerate() {
            let mut count = 1;
            let mut outcome = 0;

            while self.weak_number > outcome {
                outcome = self.numbers[start_index .. start_index+1+count].iter().sum::<i64>();
                count += 1;
            }

            if outcome == self.weak_number {
                let lowest = self.numbers[start_index .. start_index+count].iter().min().expect("No min value found in number range");
                let highest = self.numbers[start_index .. start_index+count].iter().max().expect("No max value found in number range");

                return lowest + highest;
            }
        }

        -1
    }
}

struct Input {
    filename: String,
}

impl Input {
    fn new(filename: String) -> Input {
        Input { filename }
    }

    fn get_weakness_finder(&self, preamble_range: i32) -> XMASCypherWeaknessFinder {
        let contents = self.file_contents();

        XMASCypherWeaknessFinder { preamble_range, numbers: contents.lines().map(|line| line.parse::<i64>().expect("Number could not be parsed")).collect() }
    }

    fn get_encryption_cracker(&self, weak_number: i64) -> XMASEncryptionCracker {
        let contents = self.file_contents();

        XMASEncryptionCracker { weak_number, numbers: contents.lines().map(|line| line.parse::<i64>().expect("Number could not be parsed")).collect() }
    }

    fn file_contents(&self) -> String {
        println!("Loading contents from file: {}", self.filename);

        return fs::read_to_string(&self.filename).expect("Something went wrong loading contents from file");
    }
}

fn main() {
    let input_filename = env::args().nth(1).unwrap_or("input.txt".to_string());
    let preamble_range = if input_filename == "sample.txt" { 5 } else { 25 };
    let input = Input::new(input_filename);

    let mut weakness_finder = input.get_weakness_finder(preamble_range);
    let weak_number = weakness_finder.find();
    let mut encryption_cracker = input.get_encryption_cracker(weak_number);

    println!("Answer one: {}", weak_number);
    println!("Answer two: {}", encryption_cracker.crack());
}
