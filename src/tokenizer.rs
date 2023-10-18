use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenType {
    Number,
    String,
    Identifier,
    Equals,
    OpenParen,
    CloseParen,
    BinaryOperator,
    UnaryOperator,
    LogicalOperator,
    ComparisonOperator,
    Whitespace,
    SemiColon,
    OpenComment,
    CloseComment,

    Comma,
    ColonColon,
    Colon,
    Dot,
    LSquirly,
    RSquirly,
    OpenBracket,
    CloseBracket,

    Function,
    Return,
    Let,
    Constant,
    If,
    Else,
    While,
    Loop,
    ForEach,
    For,
    In,
    Break,
    Not,
    And,
    Or,
    Xor,
    Import,
    Class,

    EndOfFile,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub r#type: TokenType,
    pub value: String,
}

lazy_static::lazy_static! {
    pub static ref KEYWORDS: HashMap<&'static str, TokenType> = {
        let mut map = HashMap::new();
        map.insert("let", TokenType::Let);
        map.insert("const", TokenType::Constant);
        map.insert("func", TokenType::Function);
        map.insert("return", TokenType::Return);
        map.insert("if", TokenType::If);
        map.insert("else", TokenType::Else);
        map.insert("while", TokenType::While);
        map.insert("loop", TokenType::Loop);
        map.insert("foreach", TokenType::ForEach);
        map.insert("for", TokenType::For);
        map.insert("in", TokenType::In);
        map.insert("break", TokenType::Break);
        map.insert("not", TokenType::LogicalOperator);
        map.insert("and", TokenType::LogicalOperator);
        map.insert("or", TokenType::LogicalOperator);
        map.insert("import", TokenType::Import);
        map.insert("class", TokenType::Class);
        return map;
    };
}

pub fn create_token(value: &str, token_type: TokenType) -> Token {
    return Token {
        r#type: token_type,
        value: value.to_string(),
    };
}

pub fn is_alpha(src: &str) -> bool {
    return src.chars().any(|c| c.is_ascii_alphabetic() || c.is_digit(10) || c == '_');
}

pub fn is_int(src: &str) -> bool {
    return src.parse::<i64>().is_ok()
}

pub fn is_float(src: &str) -> bool {
    return src.parse::<f64>().is_ok()
}

pub fn is_whitespace(src: &str) -> bool {
    return src == " " || src == "\t" || src == "\n" || src == "\r"
}

pub fn tokenize(source_code: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut src: Vec<char> = source_code.chars().collect();

    while !src.is_empty() {
        if src[0] == '(' {
            tokens.push(create_token(&src[0].to_string(), TokenType::OpenParen));
            src = src[1..].to_vec();
        } else if src[0] == ')' {
            tokens.push(create_token(&src[0].to_string(), TokenType::CloseParen));
            src = src[1..].to_vec();
        } else if src[0] == '{' {
            tokens.push(create_token(&src[0].to_string(), TokenType::LSquirly));
            src = src[1..].to_vec();
        } else if src[0] == '}' {
            tokens.push(create_token(&src[0].to_string(), TokenType::RSquirly));
            src = src[1..].to_vec();
        } else if src[0] == '[' {
            tokens.push(create_token(&src[0].to_string(), TokenType::OpenBracket));
            src = src[1..].to_vec();
        } else if src[0] == ']' {
            tokens.push(create_token(&src[0].to_string(), TokenType::CloseBracket));
            src = src[1..].to_vec();
        } else if "+-*/%&|^".contains(src[0]) {
            if (src[0] == '-' && is_int(&src[1..].iter().collect::<String>()))
                || (src[0] == '-' && is_float(&src[1..].iter().collect::<String>()))
                || (src[0] == '-' && is_alpha(&src[1..].iter().collect::<String>()))
            {
                tokens.push(create_token(&src[0].to_string(), TokenType::UnaryOperator));
                src = src[1..].to_vec();
                continue;
            }
            if src[0] == '+' && src[1] == '+' {
                tokens.push(create_token("++", TokenType::UnaryOperator));
                src = src[2..].to_vec();
                continue;
            }
            if src[0] == '-' && src[1] == '-' {
                tokens.push(create_token("--", TokenType::UnaryOperator));
                src = src[2..].to_vec();
                continue;
            }
            if src[0] == '*' && src[1] == '*' {
                tokens.push(create_token("**", TokenType::BinaryOperator));
                src = src[2..].to_vec();
                continue;
            }
            if src[0] == '/' && src[1] == '*' {
                tokens.push(create_token("/*", TokenType::OpenComment));
                src = src[2..].to_vec();
                continue;
            }
            if src[0] == '*' && src[1] == '/' {
                tokens.push(create_token("*/", TokenType::CloseComment));
                src = src[2..].to_vec();
                continue;
            }
            if src[0] == '/' && src[1] == '/' {
                tokens.push(create_token("//", TokenType::BinaryOperator));
                src = src[2..].to_vec();
                continue;
            }
            tokens.push(create_token(&src[0].to_string(), TokenType::BinaryOperator));
            src = src[1..].to_vec();
        } else if src[0] == '=' {
            tokens.push(create_token(&src[0].to_string(), TokenType::Equals));
            src = src[1..].to_vec();
        } else if src[0] == '>' {
            if src[1] == '>' {
                tokens.push(create_token(">>", TokenType::BinaryOperator));
                src = src[2..].to_vec();
                continue;
            }
            tokens.push(create_token(&src[0].to_string(), TokenType::ComparisonOperator));
            src = src[1..].to_vec();
        } else if src[0] == '<' {
            if src[1] == '<' {
                tokens.push(create_token("<<", TokenType::BinaryOperator));
                src = src[2..].to_vec();
                continue;
            }
            tokens.push(create_token(&src[0].to_string(), TokenType::ComparisonOperator));
            src = src[1..].to_vec();
        } else if src[0] == '>' && src[1] == '=' {
            tokens.push(create_token(">=", TokenType::ComparisonOperator));
            src = src[2..].to_vec();
        } else if src[0] == '<' && src[1] == '=' {
            tokens.push(create_token("<=", TokenType::ComparisonOperator));
            src = src[2..].to_vec();
        } else if src[0] == '=' && src[1] == '=' {
            tokens.push(create_token("==", TokenType::ComparisonOperator));
            src = src[2..].to_vec();
        } else if src[0] == '!' && src[1] == '=' {
            tokens.push(create_token("!=", TokenType::ComparisonOperator));
            src = src[2..].to_vec();
        } else if src[0] == ';' {
            tokens.push(create_token(&src[0].to_string(), TokenType::SemiColon));
            src = src[1..].to_vec();
        } else if src[0] == ',' {
            tokens.push(create_token(&src[0].to_string(), TokenType::Comma));
            src = src[1..].to_vec();
        } else if src[0] == '.' {
            tokens.push(create_token(&src[0].to_string(), TokenType::Dot));
            src = src[1..].to_vec();
        } else if src[0] == ':' {
            if src[1] == ':' {
                tokens.push(create_token("::", TokenType::ColonColon));
                src = src[2..].to_vec();
                continue;
            }
            tokens.push(create_token(&src[0].to_string(), TokenType::Colon));
            src = src[1..].to_vec();
        } else if src[0] == '\"' {
            src = src[1..].to_vec();
            let mut string_value = String::new();
            while !src.is_empty() && src[0] != '\"' {
                string_value.push(src[0]);
                src = src[1..].to_vec();
            }

            if src.is_empty() {
                println!("Error: Unterminated string");
                std::process::exit(0);
            }

            tokens.push(create_token(&string_value, TokenType::String));
            src = src[1..].to_vec();
        } else {
            if is_int(&src[0].to_string()) || (src[0] == '-' && is_int(&src[1..].iter().collect::<String>())) {
                let mut num = String::new();
                let mut is_float_num = false;

                while !src.is_empty() && (is_int(&src[0].to_string())
                    || (!is_float_num
                        && src[0] == '.'
                        && !src[1..].is_empty()
                        && is_int(&src[1..].iter().collect::<String>())))
                {
                    if src[0] == '.' {
                        is_float_num = true;
                    }
                    num.push(src[0]);
                    src = src[1..].to_vec();
                }

                while !src.is_empty() && is_int(&src[0].to_string()) {
                    num.push(src[0]);
                    src = src[1..].to_vec();
                }

                tokens.push(create_token(&num, TokenType::Number));
            } else if is_alpha(&src[0].to_string()) {
                let mut identifier = String::new();
                while !src.is_empty() && is_alpha(&src[0].to_string()) {
                    identifier.push(src[0]);
                    src = src[1..].to_vec();
                }

                if let Some(&reserved) = KEYWORDS.get(&identifier as &str) {
                    tokens.push(create_token(&identifier, reserved));
                } else {
                    tokens.push(create_token(&identifier, TokenType::Identifier));
                }
            } else if is_whitespace(&src[0].to_string()) {
                src = src[1..].to_vec();
            } else {
                println!("Error: Invalid character '{}'", src[0]);
                std::process::exit(0);
            }
        }
    }

    tokens.push(create_token("EndOfFile", TokenType::EndOfFile));
    return tokens;
}

