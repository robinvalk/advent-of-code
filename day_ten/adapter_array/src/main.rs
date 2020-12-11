use std::fs;
use std::env;
use std::collections::HashMap;

struct JoltageDifferenceCalculator {
    adapters: Vec<i32>,
}

impl JoltageDifferenceCalculator {
    fn calculate(&mut self) -> i32 {
        self.adapters.sort();

        println!("{:?}", self.adapters);

        let mut current_joltage = 0;
        let mut one_diff = 0;
        let mut three_diff = 1;

        for adapter in self.adapters.iter() {
            let diff = adapter - current_joltage;

            if diff % 3 == 0 {
                three_diff += 1;
            } else if diff % 2 == 0 {

            } else {
                one_diff += 1;
            }

            current_joltage = *adapter;
        }

        println!("{}", self.adapters.len());
        println!("one diff: {}, three diff: {}", one_diff, three_diff);
        println!("{}", self.adapters.iter().sum::<i32>());

        one_diff * three_diff
    }
}

struct DistinctAdapterCombinationCountCalculator {
    adapters: Vec<i32>,
}

impl DistinctAdapterCombinationCountCalculator {
    fn calculate(&mut self) -> u128 {
        let mut history: HashMap<i32, u128> = HashMap::new();

        self.adapters.push(0);
        self.adapters.push(self.adapters.iter().max().unwrap() + 3);
        self.adapters.sort();

        self.count_next_options(0, &mut history)
    }

    fn count_next_options(&self, index: i32, history: &mut HashMap<i32, u128>) -> u128 {
        if index == (self.adapters.len() as i32 - 1) {
            return 1;
        }

        if history.get(&index).is_some() {
            return *history.get(&index).unwrap();
        }

        let mut total: u128 = 0;
        let start_index = (index + 1) as usize;
        let adapter = self.adapters[index as usize];

        for (mid_index, mid_adapter) in self.adapters[start_index ..].iter().enumerate() {
            if *mid_adapter - adapter > 3 || *mid_adapter == adapter {
                break;
            }

            let next_index = start_index as i32 + mid_index as i32;
            total += self.count_next_options(next_index, history);
        }

        history.insert(index, total);

        total
    }
}

struct Input {
    filename: String,
}

impl Input {
    fn new(filename: String) -> Input {
        Input { filename }
    }

    fn get_jolt_difference_calculator(&self) -> JoltageDifferenceCalculator {
        let contents = self.file_contents();

        JoltageDifferenceCalculator { adapters: contents.lines().map(|line| line.parse::<i32>().unwrap()).collect::<Vec<i32>>() }
    }

    fn get_distinct_combi_calculator(&self) -> DistinctAdapterCombinationCountCalculator {
        let contents = self.file_contents();

        DistinctAdapterCombinationCountCalculator {
            adapters: contents.lines().map(|line| line.parse::<i32>().unwrap()).collect::<Vec<i32>>()
        }
    }

    fn file_contents(&self) -> String {
        println!("Loading contents from file: {}", self.filename);

        return fs::read_to_string(&self.filename).expect("Something went wrong loading contents from file");
    }
}

fn main() {
    let input_filename = env::args().nth(1).unwrap_or("input.txt".to_string());
    let input = Input::new(input_filename);

    let mut joltage_difference_calculator = input.get_jolt_difference_calculator();
    let mut distinct_combi_calculator = input.get_distinct_combi_calculator();

    println!("Answer one: {}", joltage_difference_calculator.calculate());
    println!("Answer two: {}", distinct_combi_calculator.calculate());
}
