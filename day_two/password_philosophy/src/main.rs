use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let input_file: String = get_file_from_args(&args);
    let input_string = load_contents_from_file(input_file);

    let number_of_valid_passwords = count_valid_passwords(input_string.to_string());
    println!("Answer one: {}", number_of_valid_passwords);

    let number_of_valid_passwords = count_valid_passwords_two(input_string.to_string());
    println!("Answer two: {}", number_of_valid_passwords);
}

fn count_valid_passwords(input: String) -> u32 {
    let mut count = 0;

    for line in input.lines() {
        let split: Vec<&str> = line.split(": ").collect();
        let policy_string = &split[0];
        let policy = parse_policy_string(policy_string.to_string());
        let password = split[1].to_string();
    
        if password_matches_policy(policy, password) {
            count += 1;
        }
    }

    return count;
}

fn count_valid_passwords_two(input: String) -> u32 {
    let mut count = 0;

    for line in input.lines() {
        let split: Vec<&str> = line.split(": ").collect();
        let policy_string = &split[0];
        let policy = parse_policy_string_two(policy_string.to_string());
        let password = split[1].to_string();
    
        if password_matches_policy_two(policy, password) {
            count += 1;
        }
    }

    return count;
}

struct CountPasswordPolicy {
    min: u32,
    max: u32,
    character: char,
}

struct PositionPasswordPolicy {
    pos_one: u32,
    pos_two: u32,
    character: char,
}

fn parse_policy_string(policy_string: String) -> CountPasswordPolicy {
    let policy_parts: Vec<&str> = policy_string.split_whitespace().collect();
    let (limits, character) = (policy_parts[0], policy_parts[1].parse().unwrap());
    let limit_parts: Vec<&str> = limits.split("-").collect();
    let (min, max) = (limit_parts[0].parse::<u32>().unwrap(), limit_parts[1].parse::<u32>().unwrap());

    return CountPasswordPolicy { min, max, character };
}


fn parse_policy_string_two(policy_string: String) -> PositionPasswordPolicy {
    let policy_parts: Vec<&str> = policy_string.split_whitespace().collect();
    let (limits, character) = (policy_parts[0], policy_parts[1].parse().unwrap());
    let limit_parts: Vec<&str> = limits.split("-").collect();
    let (pos_one, pos_two) = (limit_parts[0].parse::<u32>().unwrap(), limit_parts[1].parse::<u32>().unwrap());

    return PositionPasswordPolicy { pos_one, pos_two, character };
}

fn password_matches_policy(policy: CountPasswordPolicy, password: String) -> bool {
    let chars = password.chars().filter(|&el|el == policy.character);
    let count: u32 = chars.count() as u32;

    return count >= policy.min && count <= policy.max;
}

fn password_matches_policy_two(policy: PositionPasswordPolicy, password: String) -> bool {
    let pos_one_valid = password.chars().nth(policy.pos_one as usize - 1).unwrap() == policy.character;
    let pos_two_valid = password.chars().nth(policy.pos_two as usize - 1).unwrap() == policy.character;

    return pos_one_valid != pos_two_valid && (pos_one_valid || pos_two_valid);
}

fn get_file_from_args(args: &[String]) -> String {
    if args.len() > 1 {
        return args[1].clone();
    }

    return "input.txt".to_string();
}

fn load_contents_from_file(filename: String) -> String {
    println!("Loading input from file: {}", filename);
    let input_string = fs::read_to_string(filename)
        .expect("Something went wrong reading the input file");

    return input_string;
}
