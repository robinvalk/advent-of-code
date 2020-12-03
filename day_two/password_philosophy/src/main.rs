use std::env;
use std::fs;

trait Validator<T> {
    fn new(line: &str) -> T;
    fn valid(&self) -> bool;
}

struct CharacterCountValidator {
    character: char,
    min: u32,
    max: u32,
    password: String,
}

impl Validator<CharacterCountValidator> for CharacterCountValidator {
    fn new(line: &str) -> CharacterCountValidator {
        let line_split = line.split(": ").collect::<Vec<&str>>();
        let password = line_split.last().expect("Password not found on line!").to_string();
        let char_split = line_split.first().expect("Line could not be parsed!").split(" ").collect::<Vec<&str>>();
        let character = char_split.last().expect("Character could not be found in the line!").chars().nth(0).expect("Character was empty!");
        let limits_split = char_split.first().expect("Character limits could not be found in line!").split("-").collect::<Vec<&str>>();

        let min = limits_split.first().expect("Minimal limit could not be split from input line!").to_string().parse().unwrap();
        let max = limits_split.last().expect("Maximum limit could not be split from input line!").to_string().parse().unwrap();

        CharacterCountValidator {
            character, min, max, password
        }
    }

    fn valid(&self) -> bool {
        let chars = self.password.chars().filter(|&el|el == self.character);
        let count: u32 = chars.count() as u32;

        return count >= self.min && count <= self.max;
    }
}

struct CharacterPositionValidator {
    character: char,
    positions: [usize; 2],
    password: String,
}

impl CharacterPositionValidator {
    fn position_valid(&self, position: usize) -> Option<bool> {
        self.password.chars()
            .nth(position)
            .map(|char_on_position| char_on_position == self.character)
    }
}

impl Validator<CharacterPositionValidator> for CharacterPositionValidator {
    fn new(line: &str) -> CharacterPositionValidator {
        let line_split = line.split(": ").collect::<Vec<&str>>();
        let password = line_split.last().expect("Password not found on line!").to_string();
        let char_split = line_split.first().expect("Line could not be parsed!").split(" ").collect::<Vec<&str>>();
        let character = char_split.last().expect("Character could not be found in the line!").chars().nth(0).expect("Character was empty!");
        let positions_split = char_split.first().expect("Character positions could not be found in line!").split("-").collect::<Vec<&str>>();


        let positions = [
            positions_split.first().expect("First position could not be found in line").parse::<usize>().expect("Could not parse first position!"),
            positions_split.last().expect("First position could not be found in line").parse::<usize>().expect("Could not parse second position!"),
        ];

        CharacterPositionValidator {
            character, password, positions
        }
    }

    fn valid(&self) -> bool {
        let mut results = [false, false];

        for (index, &position) in self.positions.iter().enumerate() {
            match self.position_valid(position) {
                Some(is_valid) => results[index] = is_valid,
                None => return false,
            }
        }

        results.len() == self.positions.len() && results.iter().filter(|result|**result).count() == 1
    }
}

struct Challenge<T> {
    challenges: Vec<T>
}

trait ChallengeAnswerer<T> {
    fn new<'a, I>(lines: I) -> Challenge<T> where I: IntoIterator<Item = &'a str>;
    fn answer(&self) -> usize;
}

impl ChallengeAnswerer<CharacterPositionValidator> for Challenge<CharacterPositionValidator> {
    fn new<'a, I>(lines: I) -> Challenge<CharacterPositionValidator>
    where
        I: IntoIterator<Item = &'a str>,
    {
        Challenge::<CharacterPositionValidator> {
            challenges: lines.into_iter().map(|line|CharacterPositionValidator::new(line)).collect()
        }
    }

    fn answer(&self) -> usize {
        self.challenges.iter().filter(|validator| validator.valid()).count()
    }
}

impl ChallengeAnswerer<CharacterCountValidator> for Challenge<CharacterCountValidator> {
    fn new<'a, I>(lines: I) -> Challenge<CharacterCountValidator>
    where
        I: IntoIterator<Item = &'a str>,
    {
        Challenge::<CharacterCountValidator> {
            challenges: lines.into_iter().map(|line|CharacterCountValidator::new(line)).collect()
        }
    }

    fn answer(&self) -> usize {
        self.challenges.iter().filter(|validator| validator.valid()).count()
    }
}

struct Input {
    filename: String,
}

impl Input {
    fn new(filename: String) -> Input {
        Input { filename }
    }

    fn parse_as_character_position_challenge(&self) -> Challenge<CharacterPositionValidator> {
        let contents = self.file_contents();
        let lines: Vec<&str> = contents.lines().collect();

        Challenge::<CharacterPositionValidator>::new(lines)
    }

    fn parse_as_character_count_challenge(&self) -> Challenge<CharacterCountValidator> {
        let contents = self.file_contents();
        let lines: Vec<&str> = contents.lines().collect();

        Challenge::<CharacterCountValidator>::new(lines)
    }

    fn file_contents(&self) -> String {
        println!("Loading contents from file: {}", self.filename);

        return fs::read_to_string(&self.filename).expect("Something went wrong loading contents from file");
    }
}

fn main() {
    let input = Input::new(env::args().nth(1).unwrap_or("input.txt".to_string()));
    let challenge_one = input.parse_as_character_position_challenge();
    let challenge_two = input.parse_as_character_count_challenge();

    println!("Answer one: {}", challenge_one.answer());
    println!("Answer two: {}", challenge_two.answer());
}
