use std::collections::HashMap;
use crate::instructions::Instruction;
use crate::lex::Token;

/// Parsing needs to be done in 2 phases as we don't know the location of labels
///
/// This enum is the result of the first phase of parsing
pub enum Statement {
    // fully parsed instructions
    Instruction(Instruction),
    Jump(String),
}

pub fn parse(tokens: Vec<Token>) -> Vec<Instruction> {
    let mut instructions: Vec<Statement>  = Vec::new();
    // storing the location of every label
    let mut labels: HashMap<String, usize> = HashMap::new();

    resolve_addresses(instructions, labels)
}

pub fn resolve_addresses(instructions: Vec<Statement>, labels: HashMap<String, usize>) -> Vec<Instruction> {
    let mut resolved_instructions: Vec<Instruction> = Vec::new();

    resolved_instructions
}
