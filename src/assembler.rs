use regex::Regex;
use std::collections::HashMap;

pub trait Assemblable {
    fn assemble(self) -> Vec<String>;
}

impl Assemblable for Vec<String> {
    fn assemble(self) -> Vec<String> {
        let clean_regex = Regex::new("(//.*$)|(\\s)").unwrap();
        let label_regex = Regex::new("\\(.*\\)").unwrap();
        let mut labels: HashMap<String, i32> = HashMap::new();
        let mut count = 15;
        self.iter()
            .map(|x| clean_regex.replace_all(x, "").to_string())
            .filter(|x| x != "")
            .enumerate()
            .filter_map(|(_index, x)| {
                if label_regex.is_match(&x) {
                    let _key = &x[1..x.len()];

                    None
                } else {
                    Some(x)
                }
            })
            .map(|x| {
                if &x[0..1] == "@" && !(&x[1..]).parse::<u64>().is_ok() {
                    // TODO make a first run through to collapse () labels into numbers
                    let index = match labels.get(&x[1..]) {
                        Some(T) => *T,
                        None => {
                            count += 1;
                            labels.insert(x[1..].to_string(), count);
                            count
                        }
                    };
                    format!("@{}", index)
                } else {
                    x
                }
            })
            .map(|x| {
                if &x[0..1] == "@" {
                    format!(
                        "@{}",
                        match &x[1..] {
                            "SP" => "0",
                            "LCL" => "@",
                            "ARG" => "2",
                            "THIS" => "3",
                            "THAT" => "4",
                            "SCREEN" => "16384",
                            "KBD" => "24576",
                            symbol if symbol.parse::<i32>().is_ok() => symbol,
                            symbol => panic!("Could not parse symbol: {}", symbol),
                        }
                    )
                } else {
                    x
                }
            })
            // .map(|x| match x {
            //     instr if &instr[0..1] == "@" => (&x[1..]).parse::<u16>().unwrap(),
            //
            //     _ => panic!("Could not parse instruction")
            // })
            .collect::<Vec<String>>()
    }
}
