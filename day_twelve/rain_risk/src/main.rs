use std::fs;
use std::env;

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
    heading: char,
}

struct ShipNavigator {
    position: Position,
    instuctions: Vec<String>,
    headings: [char;4],
}

impl ShipNavigator {
    fn navigate(&mut self) -> i32 {
        for instuction in self.instuctions.iter() {
            let chars: Vec<char> = instuction.chars().collect();
            let mut action = *chars.get(0).unwrap();
            let range = chars[1..].iter().collect::<String>().parse::<i32>().unwrap();

            println!("Action: {}, Range: {}", action, range);
            if action == 'F' {
                action = self.position.heading;
            }

            if action == 'N' {
                self.position.y += range;
            } else if action == 'E' {
                self.position.x += range;
            } else if action == 'S' {
                self.position.y -= range;
            } else if action == 'W' {
                self.position.x -= range;
            }

            if action == 'R' || action == 'L' {
                let current_index = self.headings.iter().position(|&wind_direction| wind_direction == self.position.heading).expect("Bug in code");

                let mut next_index = current_index as i32 + 4;
                let amount = if range == 90 { 1 } else if range == 180 { 2 } else { 3 };
                if action == 'R' {
                    next_index += amount;
                } else {
                    next_index -= amount;
                }

                self.position.heading = self.headings[(next_index%4) as usize];
            }
        }

        println!("{:?}", self.position);

        self.position.x.abs() + self.position.y.abs()
    }
}

struct WaypointNavigator {
    waypoint_position: Position,
    ship_position: Position,
    instuctions: Vec<String>,
    headings: [char;4],
}

impl WaypointNavigator {
    fn navigate(&mut self) -> i32 {
        for instuction in self.instuctions.iter() {
            let chars: Vec<char> = instuction.chars().collect();
            let action = *chars.get(0).unwrap();
            let range = chars[1..].iter().collect::<String>().parse::<i32>().unwrap();

            println!("Action: {}, Range: {}", action, range);
            if action == 'F' {
                self.ship_position.x += self.waypoint_position.x * range;
                self.ship_position.y += self.waypoint_position.y * range;
            }

            if action == 'N' {
                self.waypoint_position.y += range;
            } else if action == 'E' {
                self.waypoint_position.x += range;
            } else if action == 'S' {
                self.waypoint_position.y -= range;
            } else if action == 'W' {
                self.waypoint_position.x -= range;
            }

            if action == 'R' || action == 'L' {
                let amount = if range == 90 { 1 } else if range == 180 { 2 } else { 3 };
                let mut next_index = 4;
                if action == 'R' {
                    next_index += amount;
                } else {
                    next_index -= amount;
                }

                let relative_heading = self.headings[(next_index%4) as usize];

                let x = self.waypoint_position.x;
                let y = self.waypoint_position.y;

                if relative_heading == 'S' {
                    self.waypoint_position.x = x * -1;
                    self.waypoint_position.y = y * -1;
                } else if relative_heading == 'E' {
                    self.waypoint_position.y = x * -1;
                    self.waypoint_position.x = y;
                } else if relative_heading == 'W' {
                    self.waypoint_position.y = x * 1;
                    self.waypoint_position.x = y * -1;
                }
            }

            println!("Waypoint: {:?}", self.waypoint_position);
            println!("Ship: {:?}", self.ship_position);
        }

        self.ship_position.x.abs() + self.ship_position.y.abs()
    }
}

struct Input {
    filename: String,
}

impl Input {
    fn new(filename: String) -> Input {
        Input { filename }
    }

    fn get_ship_navigator(&self) -> ShipNavigator {
        let contents = self.file_contents();
        let lines = contents.lines();

        ShipNavigator {
            headings: ['N', 'E', 'S', 'W'],
            position: Position { x: 0, y: 0, heading: 'E' },
            instuctions: lines.map(|line| String::from(line)).collect::<Vec<String>>(),
        }
    }

    fn get_waypoint_navigator(&self) -> WaypointNavigator {
        let contents = self.file_contents();
        let lines = contents.lines();

        WaypointNavigator {
            headings: ['N', 'E', 'S', 'W'],
            waypoint_position: Position { x: 10, y: 1, heading: 'N' },
            ship_position: Position { x: 0, y: 0, heading: 'N' },
            instuctions: lines.map(|line| String::from(line)).collect::<Vec<String>>(),
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

    let mut ship_navigator = input.get_ship_navigator();
    let mut waypoint_navigator = input.get_waypoint_navigator();

    println!("Answer one: {}", ship_navigator.navigate());
    println!("Answer two: {}", waypoint_navigator.navigate());
}
