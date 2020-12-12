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
        let mut seats_changed = 1;
        while seats_changed > 0 {
            seats_changed = self.process();
            rounds += 1;
            println!("Seats changed: {}", seats_changed);
        }

        println!("Rounds: {}", rounds);

        self.layout.iter().filter(|(_key, seat)| **seat == '#').count() as i32
    }

    fn process(&mut self) -> i32 {
        let layout_copy: HashMap<String, char> = self.get_layout_copy();
        let mut seats_changed = 0;

        for row_index in 0 .. self.size.0 { 0;
            for seat_index in 0 .. self.size.1 {
                let seat_key = self.get_seat_key(row_index as i32, seat_index as i32);
                let seat = layout_copy.get(&seat_key).unwrap();

                let occupied = self.count_adjacent_seat_occupied(&layout_copy, row_index, seat_index);
                if *seat == 'L' && occupied == 0 {
                    self.layout.insert(seat_key, '#');
                } else if *seat == '#' && occupied >= 4 {
                    self.layout.insert(seat_key, 'L');
                } else {
                    continue;
                }

                seats_changed += 1;
            }
        }

        seats_changed
    }

    fn count_adjacent_seat_occupied(&self, layout: &HashMap<String, char>, row_index: usize, seat_index: usize) -> i32 {
        let mut count = 0;

        for iter_row_index in [-1, 0, 1].iter() {
            for iter_seat_index in [-1, 0, 1].iter() {
                if *iter_row_index == 0 && *iter_seat_index == 0 {
                    continue;
                }

                let ri = row_index as i32 + iter_row_index;
                let ci = seat_index as i32 + iter_seat_index;

                if ri < 0 || ri >= self.size.0 as i32 {
                    continue;
                }

                if ci < 0 || ci >= self.size.1 as i32 {
                    continue;
                }

                let seat_key = self.get_seat_key(ri, ci);
                if *layout.get(&seat_key).unwrap() == '#' {
                    count += 1;
                }
            }
        }

        count
    }

    fn get_layout_copy(&self) -> HashMap<String, char>{
        let mut layout_copy: HashMap<String, char> = HashMap::new();

        for (key, value) in self.layout.iter() {
            layout_copy.insert(key.as_str().to_string(), *value);
        }

        layout_copy
    }

    fn get_seat_key(&self, row_index: i32, seat_index: i32) -> String {
        format!("{}{}", row_index, seat_index)
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

    println!("Answer one: {}", occupied_seat_counter.count())
}
