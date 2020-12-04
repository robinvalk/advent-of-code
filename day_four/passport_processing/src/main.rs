use std::env;
use std::fs;
use std::collections::HashMap;
use regex::Regex;

struct Passport {
    byr: i32,
    cid: Option<i32>,
    ecl: String,
    eyr: i32,
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
        let is_valid = Passport::validate_structure(&data);

        if !is_valid {
            return None;
        }

        Some(
            Passport {
                byr: data.get("byr").expect("Passport was invalid, byr missing!").parse::<i32>().expect("Couldn't parse byr value"),
                cid: data.get("cid").map(|cid| cid.parse::<i32>().expect("Couldn't parse cid value")),
                ecl: data.get("ecl").expect("Passport was invalid, ecl missing!").parse::<String>().unwrap(),
                eyr: data.get("eyr").expect("Passport was invalid, eyr missing!").parse::<i32>().unwrap(),
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

    fn validate_structure(data: &HashMap<String, String>) -> bool {
        let ignored_keys = ["cid"];
        let keys = ["byr", "cid", "ecl", "eyr", "hcl", "hgt", "iyr", "pid"];

        keys.iter().filter(|key| !data.get(**key).is_some() && !ignored_keys.contains(*key)).count() == 0
    }

    fn is_valid(&self) -> bool {
        // byr
        if self.byr < 1920 || self.byr > 2002 {
            println!("byr invalid");
            return false;
        }

        // iyr
        if self.iyr < 2010 || self.iyr > 2020 {
            println!("iyr invalid");
            return false;
        }

        // eyr
        if self.eyr < 2020 || self.eyr > 2030 {
            println!("eyr invalid");
            return false;
        }

        // hgt
        let height = self.hgt[..self.hgt.len()-2].parse::<i32>().unwrap_or(0);
        if self.hgt.ends_with("cm") && (height < 150 || height > 193) {
            println!("height invalid");
            return false;
        } else if self.hgt.ends_with("in") && (height < 59 || height > 76) {
            println!("height invalid");
            return false;
        } else if !(self.hgt.ends_with("cm") || self.hgt.ends_with("in")) {
            println!("height invalid");
            return false;
        }

        // hcl
        let re_hcl = Regex::new(r"^#[a-f0-9]{6}$").unwrap();
        if !re_hcl.is_match(&self.hcl) {
            println!("hcl invalid");
            return false;
        }

        let re_ecl = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
        if !re_ecl.is_match(&self.ecl) {
            println!("ecl invalid");
            return false;
        }

        let re_pid = Regex::new(r"^[0-9]{9}$").unwrap();
        if !re_pid.is_match(&self.pid) {
            println!("pid invalid");
            return false;
        }

        return true;
    }
}

struct PassportProcessor {
    passports: Vec<Option<Passport>>,
}

impl PassportProcessor {
    fn process(&self) -> usize {
        self.passports.iter().filter(|passport| passport.is_some() && passport.as_ref().unwrap().is_valid()).count()
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
