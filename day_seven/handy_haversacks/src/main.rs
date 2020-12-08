use std::fs;
use std::env;

struct BagType {
    name: String,
    amount: i32,
}

struct BagRule {
    bag_type: BagType,
    children: Vec<BagType>,
}

impl BagRule {
    fn has_child_with_name(&self, bag_rules_processor: &BagRulesProcessor, bag_name: &str) -> bool {
        if self.bag_type.name == bag_name {
            return true;
        }

        self.children.iter()
            .map(|child| bag_rules_processor.find_rule_by_bag_type(&child))
            .filter(|rule| rule.is_some())
            .filter(|bag_rule| bag_rule.unwrap().has_child_with_name(bag_rules_processor, bag_name))
            .count() > 0
    }


}

struct BagRulesRecogniser {
    contents: String,
}

impl BagRulesRecogniser {
    fn recognise(&self) -> Vec<BagRule> {
        let mut rules = Vec::new();

        for line in self.contents.lines() {
            if line.trim().is_empty() {
                continue;
            }

            let line_split = line.trim_end_matches(".").split(" bags contain ").collect::<Vec<&str>>();

            let bag_type = BagRulesRecogniser::parse_bag_type(line_split[0]);
            let children_split = line_split[1].split(", ").collect::<Vec<&str>>();
            let children = children_split.iter().map(|child| BagRulesRecogniser::map_to_child(child)).collect();

            rules.push(BagRule { bag_type, children });
        }

        rules
    }

    fn parse_bag_type(name: &str) -> BagType {
        BagType { name: name.to_string(), amount: 0 }
    }

    fn map_to_child(line: &str) -> BagType {
        let splits = line.trim_end_matches(" bags").split(" ").collect::<Vec<&str>>();
        let name = splits[1..].join(" ");
        let amount_str = if splits[0] != "no" { splits[0] } else { "0" };
        let amount = amount_str.parse::<i32>().expect("Amount could not be parsed into integer");

        BagType { name: name.to_string(), amount }
    }
}

struct BagRulesProcessor {
    rules: Vec<BagRule>,
}

impl BagRulesProcessor {

    fn find_rule_by_bag_type(&self, bag_type: &BagType) -> Option<&BagRule> {
        self.rules.iter()
            .filter(|rule| rule.bag_type.name == bag_type.name)
            .next()
    }

    fn process(&self, bag_name: &str) -> usize {
        println!("Scanning {} rules!", self.rules.len());

        let all_rules = self.rules.iter()
            .filter(|rule| rule.has_child_with_name(&self, bag_name))
            .collect::<Vec<&BagRule>>();

        for rule in all_rules.iter() {
            println!("{}", rule.bag_type.name);
        }

        all_rules.len() - 1
    }
}

struct Input {
    filename: String,
}

impl Input {
    fn new(filename: String) -> Input {
        Input { filename }
    }

    fn load(&self) -> BagRulesProcessor {
        let contents = self.file_contents();

        let recogniser = BagRulesRecogniser { contents };

        BagRulesProcessor { rules: recogniser.recognise() }
    }

    fn file_contents(&self) -> String {
        println!("Loading contents from file: {}", self.filename);

        return fs::read_to_string(&self.filename).expect("Something went wrong loading contents from file");
    }
}

fn main() {
    let input = Input::new(
        env::args().nth(1).unwrap_or("bag_rules.txt".to_string())
    );

    let bag_rule_processor = input.load();

    println!("Answer one: {}", bag_rule_processor.process("shiny gold"));
}
