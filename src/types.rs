#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Register {
    A,
    D,
    M,
    None,
}
impl From<&str> for Register {
    fn from(val: &str) -> Self {
        return match val {
            "A" => Register::A,
            "D" => Register::D,
            "M" => Register::M,
            _ => Register::None,
        };
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Location {
    Address(u16),
    Label(String),
}

impl From<&str> for Location {
    fn from(val: &str) -> Self {
        let parsed = val.parse::<u16>();
        return match parsed {
            Ok(x) => Location::Address(x),
            Err(_) => Location::Label(val.into()),
        };
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Source {
    Register(Register),
    One,
    Zero,
    None,
}

impl From<&str> for Source {
    fn from(val: &str) -> Self {
        return match val {
            "A" => Source::Register(Register::A),
            "D" => Source::Register(Register::D),
            "M" => Source::Register(Register::M),
            "0" => Source::Zero,
            "1" => Source::One,
            _ => Source::None,
        };
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Operation {
    Nop,
    Not,
    Negative,
    Add,
    Subtract,
    And,
    Or,
    None,
}

impl From<&str> for Operation {
    fn from(val: &str) -> Self {
        return match val {
            "-" => Operation::Negative,
            "!" => Operation::Not,
            "+" => Operation::Add,
            "&" => Operation::And,
            "|" => Operation::Or,
            _ => Operation::None,
        };
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Jump {
    JGT,
    JEQ,
    JGE,
    JLT,
    JNE,
    JLE,
    JMP,
    None,
}

impl From<&str> for Jump {
    fn from(val: &str) -> Self {
        return match val.to_ascii_uppercase().as_ref() {
            "JGT" => Jump::JGT,
            "JEQ" => Jump::JEQ,
            "JGE" => Jump::JGE,
            "JLT" => Jump::JLT,
            "JNE" => Jump::JNE,
            "JLE" => Jump::JLE,
            "JMP" => Jump::JMP,
            _ => Jump::None,
        };
    }
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Computation {
    Computation(Source, Source, Operation),
    None,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Macro {
    Call(String),
    Return,
    Include(String),
    None,
}

impl From<(&str, &str)> for Macro {
    fn from(val: (&str, &str)) -> Self {
        return match (val.0.to_ascii_lowercase().as_ref(), val.1) {
            ("call", arg) => Macro::Call(arg.into()),
            ("ret", _) => Macro::Return,
            ("include", arg) => Macro::Include(arg.into()),
            _ => Macro::None,
        };
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Instruction {
    A(Location),
    C(Vec<Register>, Computation, Jump),
    Macro(Macro),
}
