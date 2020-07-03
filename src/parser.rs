use crate::types::*;
use nom::branch::alt;
use nom::bytes::complete::{tag, take, take_until, take_while};
use nom::combinator::opt;
use nom::error::{ErrorKind, ParseError, VerboseError, VerboseErrorKind};
use nom::multi::many1;
use nom::IResult;
use regex::Regex;
use std::collections::HashMap;
fn parse_a(text: &str) -> IResult<&str, Instruction, VerboseError<&str>> {
    let (text, _) = tag("@")(text)?;
    let (text, location) = take_while(|ch| ch != '\n')(text)?;
    let (text, _) = opt(tag("\n"))(text)?;
    Ok((text, Instruction::A(Location::from(location))))
}

fn parse_dest(text: &str) -> IResult<&str, Vec<Register>, VerboseError<&str>> {
    let (text, dests): (&str, Vec<&str>) =
        many1(alt((tag("A"), tag("D"), tag("D"))))(text)?;
    let (text, _) = opt(tag("="))(text)?;
    Ok((text, dests.into_iter().map(|x| x.into()).collect()))
}

fn parse_computation(text: &str) -> IResult<&str, Computation, VerboseError<&str>> {
    let (text, first_char) = take(1usize)(text)?;

    let op = Operation::from(first_char);
    if op != Operation::None {
        let (text, second_char) = take(1usize)(text)?;

        let lhs = Source::from(second_char);
        if lhs == Source::None {
            Err(nom::Err::Error(VerboseError::from_error_kind(
                text,
                ErrorKind::Char,
            ))) // TODO fix this error
        } else {
            Ok((text, Computation::Computation(lhs, Source::None, op)))
        }
    } else {
        let lhs = Source::from(first_char);
        let end = alt((tag(";"), tag("\n")))(text);
        if let Ok(_) = end {
            let (text, _) = end?;
            Ok((
                text,
                Computation::Computation(lhs, Source::None, Operation::None),
            ))
        } else {
            let (text, second_char) = take(1usize)(text)?;
            let (text, third_char) = take(1usize)(text)?;
            Ok((
                text,
                Computation::Computation(lhs, third_char.into(), second_char.into()),
            ))
        }
    }
}

fn parse_jmp(text: &str) -> IResult<&str, Jump, VerboseError<&str>> {
    Ok((text, text.into()))
}

fn parse_c(text: &str) -> IResult<&str, Instruction, VerboseError<&str>> {

    let (text, dest) = parse_dest(text)?;
    let (text, computation) = parse_computation(text)?;
    let (text, jmp) = parse_jmp(text)?;

    Ok((text, Instruction::C(dest, computation, jmp)))
}

fn parse_instruction(text: &str) -> IResult<&str, Instruction, VerboseError<&str>> {

    let (text, instr) = alt((parse_a, parse_c))(text)?;
    Ok((text, instr))
}

pub fn parse(asm: String) -> Vec<Instruction> {
    vec![Instruction::A(Location::Address(0))]
}

#[cfg(test)]
mod tests {
    use crate::parser::parse_a;
    use crate::types::{*};

    #[test]
    fn parses_a() {
        assert_eq!(parse_a("@1"), Ok(("", Instruction::A(Location::Address(1)))));
        assert_eq!(parse_a("@64"), Ok(("", Instruction::A(Location::Address(64)))));
        assert_eq!(parse_a("@test"), Ok(("", Instruction::A(Location::Label("test".into())))));
        assert_eq!(parse_a("@TeSt"), Ok(("", Instruction::A(Location::Label("TeSt".into())))));
        assert_eq!(parse_a("@test\n"), Ok(("", Instruction::A(Location::Label("test".into())))));
        assert_eq!(parse_a("@test\n\n\n"), Ok(("\n\n", Instruction::A(Location::Label("test".into())))));
    }
}