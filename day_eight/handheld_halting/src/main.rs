use std::fs;
use std::env;
use std::cell::Cell;

#[derive(Clone)]
struct CPU {
    accumulator: Cell<i32>,
    pointer: Cell<i32>,
    instructions: Vec<Instruction>,
    instructions_ran: Vec<i32>,
}

impl CPU {
    fn new(instructions: Vec<Instruction>) -> CPU {
        CPU {
            instructions,
            pointer: Cell::new(0),
            accumulator: Cell::new(0),
            instructions_ran: Vec::new(),
        }
    }

    fn run(&mut self) -> bool {
        while (self.pointer.get() as usize) < self.instructions.len() {
            if self.instructions_ran.contains(&self.pointer.get()) {
                return false;
            }

            let pointer = self.pointer.get();
            let instruction = self.instructions.get(self.pointer.get() as usize).unwrap();

            self.run_instruction(instruction);

            self.instructions_ran.push(pointer);
        }

        return true;
    }

    fn run_instruction(&self, instruction: &Instruction) {
        println!("{} {} {} {}", self.pointer.get(), instruction.operation, instruction.argument, self.accumulator.get());

        match instruction.operation.as_str() {
            "acc" => {
                self.accumulator.set(self.accumulator.get() + instruction.argument);
            },
            "jmp" => {
                self.pointer.set(self.pointer.get() + instruction.argument);
                return;
            },
            _ => {

            }
        }

        self.pointer.set(self.pointer.get() + 1);
    }
}

#[derive(Clone)]
struct Instruction {
    operation: String,
    argument: i32,
}

struct InstructionParser {

}

impl InstructionParser {
    fn parse(line: &str) -> Instruction {
        let args = line.split(" ").collect::<Vec<&str>>();

        Instruction {
            operation: args[0].to_string(),
            argument: args[1].parse::<i32>().expect("argument of instruction is invalid")
        }
    }
}

struct CPUBruteforcer {
    cpus: Vec<CPU>
}

impl CPUBruteforcer {
    fn bruteforce(&mut self) -> i32 {
        for cpu in &mut self.cpus {
            let success = cpu.run();
            if success {
                return cpu.accumulator.get();
            }
        }

        return -999;
    }
}

struct Input {
    filename: String,
}

impl Input {
    fn new(filename: String) -> Input {
        Input { filename }
    }

    fn load(&self) -> CPU {
        let contents = self.file_contents();
        let instructions: Vec<_> = contents.lines()
            .map(|line| InstructionParser::parse(line))
            .collect();

        CPU::new(instructions)
    }

    fn load_bruteforcer(&self) -> CPUBruteforcer {
        let contents = self.file_contents();
        let instructions: Vec<_> = contents.lines()
            .map(|line| InstructionParser::parse(line))
            .collect();

        let mut cpus = Vec::new();

        for (index, _instr) in instructions.clone().iter().enumerate() {
            let mut clone = instructions.clone();

            match clone[index].operation.as_str() {
                "jmp" => clone[index].operation = "nop".to_string(),
                "nop" => clone[index].operation = "jmp".to_string(),
                _ => {

                }
            }

            cpus.push(CPU::new(clone))
        }


        CPUBruteforcer {
            cpus
        }
    }

    fn file_contents(&self) -> String {
        println!("Loading contents from file: {}", self.filename);

        return fs::read_to_string(&self.filename).expect("Something went wrong loading contents from file");
    }
}

fn main() {
    let input = Input::new(
        env::args().nth(1).unwrap_or("input.txt".to_string())
    );

    let mut cpu = input.load();
    cpu.run();

    let mut cpu_bruteforcer = input.load_bruteforcer();

    println!("Answer one: {}", cpu.accumulator.get());
    println!("Answer two: {}", cpu_bruteforcer.bruteforce());
}
