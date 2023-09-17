use std::iter::Peekable;
use std::str::Chars;

#[derive(PartialEq, Debug)]
pub enum Token {
    Register(u8),
    Number(u8),
    Label(String),
    Instruction(Mnemonic),
    Directive(Directive),
    Comma,
    Colon,
    Hash,
    Newline,
    Eof,
}

#[derive(PartialEq, Debug)]
pub enum Mnemonic {
    Inc,
    Dec,
    Add,
    Nop,
}

#[derive(PartialEq, Debug)]
pub enum Directive {
    Include,
    Global,
    End,
}


pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            'r' => {
                let number = read_number(&mut chars, None);
                tokens.push(Token::Register(number));
            }
            '0'..='9' => {
                let number = read_number(&mut chars, Some(c));
                tokens.push(Token::Number(number));
            }
            'a'..='z' | 'A'..='Z' => {
                let mut str= String::new();
                str.push(c);
                while let Some(&c) = chars.peek() {
                    if c.is_alphanumeric() {
                        str.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }

                tokens.push(word_to_token(&str));
            }
            ',' => tokens.push(Token::Comma),
            ':' => tokens.push(Token::Colon),
            '#' => tokens.push(Token::Hash),
            '\n' => tokens.push(Token::Newline),
            _ => {}
        }
    }

    tokens.push(Token::Eof);

    tokens
}

fn word_to_token(str: &String) -> Token {
    match str.as_str() {
        "inc" => Token::Instruction(Mnemonic::Inc),
        "dec" => Token::Instruction(Mnemonic::Dec),
        "add" => Token::Instruction(Mnemonic::Add),
        "nop" => Token::Instruction(Mnemonic::Nop),
        _ => Token::Label(str.clone()),
    }
}

fn read_number(peekable: &mut Peekable<Chars>, start: Option<char>) -> u8 {
    let mut buffer = Vec::new();

    if let Some(c) = start {
        buffer.push(c);
    }

    while let Some(&c) = peekable.peek() {
        if c.is_digit(10) {
            buffer.push(peekable.next().unwrap());
        } else {
            break;
        }
    }

    buffer.iter().collect::<String>().parse::<u8>().unwrap()
}

#[cfg(test)]
mod test {
    #[test]
    fn happy() {
        let code: &str = "inc r1\ninc r2\nadd r1,r2\n";
        let tokens = super::tokenize(code);
        assert_eq!(tokens.len(), 12);
        let expected = [
            super::Token::Instruction(super::Mnemonic::Inc),
            super::Token::Register(1),
            super::Token::Newline,
            super::Token::Instruction(super::Mnemonic::Inc),
            super::Token::Register(2),
            super::Token::Newline,
            super::Token::Instruction(super::Mnemonic::Add),
            super::Token::Register(1),
            super::Token::Comma,
            super::Token::Register(2),
            super::Token::Newline,
            super::Token::Eof,
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn long_numbers() {
        let code: &str = "inc r10\ninc r12\nadd r10,r12\n";
        let tokens = super::tokenize(code);
        assert_eq!(tokens.len(), 12);
        let expected = [
            super::Token::Instruction(super::Mnemonic::Inc),
            super::Token::Register(10),
            super::Token::Newline,
            super::Token::Instruction(super::Mnemonic::Inc),
            super::Token::Register(12),
            super::Token::Newline,
            super::Token::Instruction(super::Mnemonic::Add),
            super::Token::Register(10),
            super::Token::Comma,
            super::Token::Register(12),
            super::Token::Newline,
            super::Token::Eof,
        ];
        assert_eq!(tokens, expected);
    }
}
