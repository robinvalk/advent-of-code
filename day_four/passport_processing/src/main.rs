use std::env;
use std::fs;
use std::collections::HashMap;

struct Passport {
    byr: i32,
    cid: Option<i32>,
    ecl: String,
    eyr: String,
    hcl: String,
    hgt: String,
    iyr: i32,
    pid: String,
}

impl Passport {
    fn create(line: &str) -> Option<Passport> {
        Passport::map_from_str(line)
    }

    fn map_from_str(line: &str) -> Option<Passport> {
        let data = Passport::load_data(line);
        let is_valid = Passport::validate_data(&data);

        if !is_valid {
            return None;
        }

        Some(
            Passport {
                byr: data.get("byr").expect("Passport was invalid, byr missing!").parse::<i32>().expect("Couldn't parse byr value"),
                cid: data.get("cid").map(|cid| cid.parse::<i32>().expect("Couldn't parse cid value")),
                ecl: data.get("ecl").expect("Passport was invalid, ecl missing!").parse::<String>().unwrap(),
                eyr: data.get("eyr").expect("Passport was invalid, eyr missing!").parse::<String>().unwrap(),
                hcl: data.get("hcl").expect("Passport was invalid, hcl missing!").parse::<String>().unwrap(),
                hgt: data.get("hgt").expect("Passport was invalid, hgt missing!").parse::<String>().unwrap(),
                iyr: data.get("iyr").expect("Passport was invalid, iyr missing!").parse::<i32>().expect("Couldn't parse iyr value"),
                pid: data.get("pid").expect("Passport was invalid, pid missing!").parse::<String>().expect("Couldn't parse pid value"),
            }
        )
    }

    fn load_data(line: &str) -> HashMap<String, String> {
        line
            .split(" ")
            .map(|kv| kv.split(":"))
            .map(|mut kv| (kv.next().unwrap().into(),
                           kv.next().unwrap().into()))
            .collect::<HashMap<String, String>>()
    }

    fn validate_data(data: &HashMap<String, String>) -> bool {
        let ignored_keys = ["cid"];
        let keys = ["byr", "cid", "ecl", "eyr", "hcl", "hgt", "iyr", "pid"];

        keys.iter().filter(|key| !data.get(**key).is_some() && !ignored_keys.contains(*key)).count() == 0
    }
}

struct PassportProcessor {
    passports: Vec<Option<Passport>>,
}

impl PassportProcessor {
    fn process(&self) -> usize {
        self.passports.iter().filter(|passport| passport.is_some()).count()
    }
}

struct Input {
    filename: String,
}

struct PassportRecogniser {
    contents: String,
}

impl PassportRecogniser {
    fn recognise(&self) -> Vec<Option<Passport>> {
        let passport_lines: Vec<String> = self.recognise_passport_lines();

        self.parse_passport_lines(&passport_lines)
    }

    fn parse_passport_lines(&self, lines: &Vec<String>) -> Vec<Option<Passport>> {
        lines.iter().map(|line| self.parse_passport_line(line)).collect()
    }

    fn parse_passport_line(&self, line: &String) -> Option<Passport> {
        Passport::create(&line)
    }

    fn recognise_passport_lines(&self) -> Vec<String> {
        let passport_chuncks: Vec<&str> = self.contents.split("\n\n").collect();

        passport_chuncks.iter().map(|passport_chuck| self.clear_passport_chuck(*passport_chuck)).collect()
    }

    fn clear_passport_chuck(&self, passport_chuck: &str) -> String {
        let data_chuncks: Vec<&str> = passport_chuck.split_whitespace().collect();

        data_chuncks.join(" ")
    }
}

impl Input {
    fn new(filename: String) -> Input {
        Input { filename }
    }

    fn load_as_passport_processor(&self) -> PassportProcessor {
        let contents = self.file_contents();

        let recogniser = PassportRecogniser { contents };

        PassportProcessor { passports: recogniser.recognise() }
    }

    fn file_contents(&self) -> String {
        println!("Loading contents from file: {}", self.filename);

        return fs::read_to_string(&self.filename).expect("Something went wrong loading contents from file");
    }
}

fn main() {
    let input = Input::new(
        env::args().nth(1).unwrap_or("passports.txt".to_string())
    );

    println!("Answer one: {}", input.load_as_passport_processor().process());
}
