use std::fs;
use std::env;
use std::collections::HashMap;

struct OccupiedSeatCounter {
    size: (usize, usize),
    layout: HashMap<String, char>,
}

impl OccupiedSeatCounter {
    fn count(&mut self) -> i32 {
        let mut rounds = 0;

        loop {
            let new_layout = self.process();
            rounds += 1;
            // println!("Round: {}", rounds);

            if self.layouts_equal(&new_layout) {
                break;
            } else {
                self.layout = new_layout;
            }
        }

        println!("Rounds: {}", rounds);

        self.layout.iter().filter(|(_key, seat)| *seat.clone() == '#').count() as i32
    }

    fn process(&self) -> HashMap<String, char> {
        let mut editable_layout: HashMap<String, char> = HashMap::new();

        for row_index in 0 .. self.size.0 { 0;
            for column_index in 0 .. self.size.1 {
                let seat_key = self.get_seat_key(row_index as i32, column_index as i32);
                let seat = self.layout.get(&seat_key).unwrap().clone();

                let occupied = self.count_adjacent_seat_occupied(row_index, column_index);
                if seat == 'L' && occupied == 0 {
                    editable_layout.insert(seat_key, '#');
                } else if seat == '#' && occupied >= 4 {
                    editable_layout.insert(seat_key, 'L');
                } else {
                    editable_layout.insert(seat_key, seat);
                }
            }
        }

        editable_layout
    }

    fn count_adjacent_seat_occupied(&self, row_index: usize, column_index: usize) -> i32 {
        let mut count = 0;

        for iter_row_index in [-1, 0, 1].iter() {
            for iter_column_index in [-1, 0, 1].iter() {
                if *iter_row_index == 0 && *iter_column_index == 0 {
                    continue;
                }

                let ri = row_index as i32 + iter_row_index;
                let ci = column_index as i32 + iter_column_index;

                let seat_key = self.get_seat_key(ri, ci);
                let seat_status = self.layout.get(&seat_key).unwrap_or(&'.').clone();

                if seat_status == '#' {
                    count += 1;
                }
            }
        }

        count
    }

    fn get_layout_copy(&self) -> HashMap<String, char>{
        let mut layout_copy: HashMap<String, char> = HashMap::new();

        for (key, value) in self.layout.iter() {
            layout_copy.insert(String::from(key), value.clone());
        }

        layout_copy
    }

    fn get_seat_key(&self, row_index: i32, seat_index: i32) -> String {
        format!("{}{}", row_index, seat_index)
    }

    fn layouts_equal(&self, new_layout: &HashMap<String, char>) -> bool {
        self.layout.iter().filter(|(key, value)| {
            let key_ref = &key.to_string();
            let val = new_layout.get(key_ref);

            if val.is_none() {
                println!("{}", key);
            }

            !new_layout.contains_key(key_ref) || val.is_none() || val.unwrap() != value.clone()
        }).count() == 0
    }
}

struct Input {
    filename: String,
}

impl Input {
    fn new(filename: String) -> Input {
        Input { filename }
    }

    fn get_occupied_seat_counter(&self) -> OccupiedSeatCounter {
        let contents = self.file_contents();
        let lines = contents.lines();
        let mut layout: HashMap<String, char> = HashMap::new();

        for (row_index, row) in lines.enumerate() {
            for (seat_index, seat) in row.chars().enumerate() {
                layout.insert(format!("{}{}", row_index, seat_index), seat);
            }
        }

        OccupiedSeatCounter {
            layout,
            size: (contents.lines().count(), contents.lines().last().unwrap().chars().count())
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

    let mut occupied_seat_counter = input.get_occupied_seat_counter();
    println!("Rows: {}, Columns: {}", occupied_seat_counter.size.0, occupied_seat_counter.size.1);

    println!("Answer one: {}", occupied_seat_counter.count())
}
