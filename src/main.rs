use std::{io, env, fs};
use std::io::Read;

#[macro_use]
extern crate lazy_static;

const STACK_POINTER: &str = "@16383";

static ret_count: u32 = 0;

lazy_static! {
/// HACK ASM routine to set the stack pointer pointing to the first element of the stack
/// The last available memory location (16383) is used as a stack pointer and the stack grows down from there
static ref SETUP_STACK: String = [STACK_POINTER, "D=A-1", "M=D","@0"].join("\n");

/// HACK ASM routine to point the stack pointer at the next element
static ref INCREMENT_SP: String = [STACK_POINTER, "M=M-1"].join("\n");

/// HACK ASM routine to point the stack pointer to the previous element
static ref DECREMENT_SP: String = [STACK_POINTER, "M=M+1"].join("\n");

/// HACK ASM routine to return to saved address
static ref RETURN: String = ["// RETURN FROM STORED ADDRESS", &DECREMENT_SP, STACK_POINTER, "A=M", "A=M", "D=A", "@12", "A=D+A", "0;JMP","// RETURNED"].join("\n");

/// HACK ASM routine to store the current address
static ref STORE: String = ["// STORE CURRENT ADDRESS","D=A", STACK_POINTER, "A=M", "M=D", &INCREMENT_SP, "// STORED"].join("\n");
}

/// Generate subroutine to call function at label
fn call(label: &str) -> String {
    [&["// JUMPING TO LABEL ", label].join("") as &str, &STORE, &["@", label].join("") as &str, "0;JMP"].join("\n")
}


/// expand
fn process(line: &str) -> String {
    return match line {
        l if l.to_lowercase().starts_with("#call") => call(l.split(" ").last().unwrap()),
        l if l.to_lowercase().starts_with("#ret") => RETURN.to_string(),
        _ => line.to_string()
    };
}

pub fn read_string_from_stdin() -> String {
    let mut response = String::new();
    io::stdin().read_to_string(&mut response).expect("Unable to read from stdin");
    return response;
}


fn main() {
    let args: Vec<String> = env::args().collect();

    let input = match args.len() {
        1 => read_string_from_stdin(),
        2 => fs::read_to_string(args[1].clone()).expect("Unable to read file "),
        _ => panic!("Should not have more than one argument")
    };

    println!("{}", SETUP_STACK.to_string());
    for i in input.split('\n').map(|x| process(x)) {
        println!("{}", i);
    }
}
