use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let input_file: String = get_file_from_args(&args);
    let input_string = load_contents_from_file(input_file);
    let input_vec = convert_input_to_integer_vector(input_string);
    
    for a in input_vec.iter().copied() {
        for b in input_vec.iter().copied() {
            if (a + b) == 2020 {
                println!("Answer is: {}", a*b);
                process::exit(1);
            }
        }
    }
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

fn convert_input_to_integer_vector(input_string: String) -> Vec<u32> {
    let mut input: Vec<u32> = Vec::new();

    let input_iterator = input_string.split("\n");
    for elem in input_iterator {
        input.push(elem.parse::<u32>().unwrap());
    }

    return input;
}