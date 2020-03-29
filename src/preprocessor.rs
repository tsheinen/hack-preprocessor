use std::string::ToString;
use std::fs;

const STACK_POINTER: &str = "@16383";

pub struct Preprocessor {
    /// HACK ASM routine to set the stack pointer pointing to the first element of the stack
    /// The last available memory location (16383) is used as a stack pointer and the stack grows down from there
    SETUP_STACK: String,

    /// HACK ASM routine to point the stack pointer at the next element
    INCREMENT_SP: String,

    /// HACK ASM routine to point the stack pointer to the previous element
    DECREMENT_SP: String,

    /// HACK ASM routine to return to saved address
    RETURN: String,

    /// HACK ASM routine to store the current address
    STORE: String,

    // Store list of files to include at the end
    included_files: Vec<String>,
}

impl Preprocessor {
    /// Generate subroutine to call function at label
    fn call(&self, label: &str) -> String {
        [&["// JUMPING TO LABEL ", label].join("") as &str, &self.STORE, &["@", label].join("") as &str, "0;JMP"].join("\n")
    }

    /// add included file to list of files to include and return empty string
    fn include(&mut self, line: &str) -> String {
        self.included_files.push(line.parse().unwrap());
        return String::new();
    }

    /// expand preprocessor directives
    fn process_line(&mut self, line: &str) -> String {
        return match line {
            l if l.to_lowercase().starts_with("#call") => self.call(l.split(" ").last().unwrap()),
            l if l.to_lowercase().starts_with("#ret") => self.RETURN.to_string(),
            l if l.to_lowercase().starts_with("#include") => self.include(l.split(" ").last().unwrap()),
            _ => line.to_string()
        };
    }

    pub fn process(&mut self, assembly: String) -> String {
        let mut output = String::new();
        output += &self.SETUP_STACK.to_string();

        output += &assembly.split('\n').map(|x| self.process_line(x)).collect::<Vec<String>>().join("\n");

        for i in self.included_files.clone() {
            output += &format!("// INCLUDED FILE {}", i);
            output += &fs::read_to_string(i).expect("Could not read file").split('\n').map(|x| self.process_line(x)).collect::<Vec<String>>().join("\n");
        }
        output
    }

    pub fn new() -> Preprocessor {
        let setup_stack = [STACK_POINTER, "D=A-1", "M=D", "@0\n"].join("\n");
        let increment_sp = [STACK_POINTER, "M=M-1"].join("\n");
        let decrement_sp = [STACK_POINTER, "M=M+1"].join("\n");
        let ret = ["// RETURN FROM STORED ADDRESS", &decrement_sp, STACK_POINTER, "A=M", "A=M", "D=A", "@12", "A=D+A", "0;JMP", "// RETURNED"].join("\n");
        let store = ["// STORE CURRENT ADDRESS", "D=A", STACK_POINTER, "A=M", "M=D", &increment_sp, "// STORED"].join("\n");
        return Preprocessor {
            SETUP_STACK: setup_stack,
            INCREMENT_SP: increment_sp,
            DECREMENT_SP: decrement_sp,
            RETURN: ret,
            STORE: store,
            included_files: Vec::new(),
        };
    }
}

