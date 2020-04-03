use std::string::ToString;
use std::fs;


pub trait Preprocessable {
    fn preprocess(self) -> Vec<String>;
}

impl Preprocessable for Vec<String> {
    fn preprocess(self) -> Vec<String> {
        const STACK_POINTER: &str = "@16383";

        fn process_line(included_files: &mut Vec<String>, line: &str) -> String {
            return match line {
                l if l.to_lowercase().starts_with("#call") => {
                    let label = l.split(" ").last().unwrap();
                    let increment_sp: String = [STACK_POINTER, "M=M-1"].join("\n");
                    let store: String = ["// store CURRENT ADDRESS", "D=A", STACK_POINTER, "A=M", "M=D", &increment_sp, "// STORED"].join("\n");
                    [&["// JUMPING TO LABEL ", label].join("") as &str, &store, &["@", label].join("") as &str, "0;JMP"].join("\n")
                },
                l if l.to_lowercase().starts_with("#ret") => {
                    let decrement_sp: String = [STACK_POINTER, "M=M+1"].join("\n");
                    ["// RETURN FROM STORED ADDRESS", &decrement_sp, STACK_POINTER, "A=M", "A=M", "D=A", "@12", "A=D+A", "0;JMP", "// RETURNED"].join("\n")
                }
                l if l.to_lowercase().starts_with("#include") => {
                    included_files.push(l.split(" ").last().unwrap().to_string());
                    String::new()
                },
                _ => line.to_string()
            };
        }
        let mut included_files: Vec<String> = Vec::new();

        let mut output = String::new();
        output += &[STACK_POINTER, "D=A-1", "M=D", "@0\n"].join("\n");

        output += &self.iter().map(|x| process_line(&mut included_files, x)).collect::<Vec<String>>().join("\n");

        for i in included_files.clone() {
            output += &format!("// INCLUDED FILE {}", i);
            output += &fs::read_to_string(&i).expect(&format!("Could not read file {:?}", i)).split('\n').map(|x| process_line(&mut included_files, x)).collect::<Vec<String>>().join("\n");
        }
        output.split("\n").map(|x| x.to_string()).collect::<Vec<String>>()
    }
}