use std::env;
use std::fs;

struct BoardingPass {
    identifier: [char; 10],
    row: i32,
    column: i32,
}

impl BoardingPass {
    fn get_id(&self) -> i32 {
        self.row * 8 + self.column
    }
}

struct BoardingPassParser {
    line: String,
}

impl BoardingPassParser {
    fn new(line: &str) -> BoardingPassParser {
        BoardingPassParser { line: line.to_string() }
    }

    fn parse(&self) -> BoardingPass {
        let row = self.determine_row();
        let column = self.determine_column();

        let chars = self.line.chars().collect::<Vec<char>>();
        let mut identifier: [char; 10] = Default::default();
        identifier.copy_from_slice(&chars[0..10]);

        BoardingPass { identifier, row, column }
    }

    fn determine_row(&self) -> i32 {
        let mut num_to_take = 128;
        let rows: Vec<i32> = (0..num_to_take).collect();
        let mut row_search = &rows[..];

        for path in self.line[0..7].chars() {
            num_to_take = num_to_take / 2;

            if path == 'B' {
                row_search = &row_search[row_search.len()-num_to_take as usize ..];
            } else if path == 'F' {
                row_search = &row_search[.. num_to_take as usize];
            }
        }

        row_search[0]
    }

    fn determine_column(&self) -> i32 {
        let mut num_to_take = 8;
        let columns: Vec<i32> = (0..num_to_take).collect();
        let mut column_search = &columns[..];

        for path in self.line[7..].chars() {
            num_to_take = num_to_take / 2;

            if path == 'R' {
                column_search = &column_search[column_search.len()-num_to_take as usize ..];
            } else if path == 'L' {
                column_search = &column_search[.. num_to_take as usize];
            }
        }

        column_search[0]
    }

}

trait Identifier {
    fn identify(&self) -> i32;
}

struct HighestBoardingPassIdentifier {
    boarding_passes: Vec<BoardingPass>,
}

impl Identifier for HighestBoardingPassIdentifier {
    fn identify(&self) -> i32 {
        self.boarding_passes.iter().map(|pass| pass.get_id()).max().unwrap_or(0)
    }
}

struct MyBoardingPassIdentifier {
    boarding_passes: Vec<BoardingPass>,
}

impl Identifier for MyBoardingPassIdentifier {
    fn identify(&self) -> i32 {
        let mut sorted_ids = self.boarding_passes.iter().map(|pass| pass.get_id()).collect::<Vec<i32>>();
        sorted_ids.sort();

        let mut next = sorted_ids[..1][0];

        for id in sorted_ids.iter() {
            if *id != next {
                return next;
            }

            next += 1;
        }

        return next;
    }
}

struct Input {
    filename: String,
}

impl Input {
    fn new(filename: String) -> Input {
        Input { filename }
    }

    fn get_boarding_passes(&self) -> Vec<BoardingPass> {
        let contents = self.file_contents();

        contents.lines()
            .map(|line| BoardingPassParser::new(line).parse())
            .collect::<Vec<BoardingPass>>()
    }

    fn file_contents(&self) -> String {
        println!("Loading contents from file: {}", self.filename);

        return fs::read_to_string(&self.filename).expect("Something went wrong loading contents from file");
    }
}

fn main() {
    let input = Input::new(
        env::args().nth(1).unwrap_or("passes.txt".to_string())
    );

    let highest_boarding_pass_id_identifier = HighestBoardingPassIdentifier { boarding_passes: input.get_boarding_passes() };
    let my_pass_id_identifier = MyBoardingPassIdentifier { boarding_passes: input.get_boarding_passes() };

    println!("Answer one: {}", highest_boarding_pass_id_identifier.identify());
    println!("Answer two: {}", my_pass_id_identifier.identify());
}
