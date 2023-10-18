use crate::tokenizer::{TokenType, Token};
use crate::ast;
use std::process::exit;

pub struct Parser {
    tokens: Vec<Token>,
    is_function: bool,
    is_loop: bool,
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            tokens: Vec::new(),
            is_function: false,
            is_loop: false,
        }
    }

    pub fn produce_ast(&mut self, source_code: &str) -> ast::Program {
        self.tokens = crate::tokenizer::tokenize(source_code);
        let mut program = ast::Program{
            body: Vec::new(),
        };

        while self.is_not_eof() {
            if self.at().r#type == TokenType::Whitespace {
                self.eat();
                continue;
            }

            program.body.push(self.parse_statement());
        }

        return program;
    }

    fn parse_statement(&mut self) -> Box<dyn ast::Statement> {
        match self.at().r#type {
            TokenType::OpenComment => return self.parse_comment(),
            TokenType::Let => return self.parse_variable_declaration(),
            TokenType::Constant => return self.parse_variable_declaration(),
            TokenType::Function => return self.parse_function_declaration(),
            TokenType::Return => {
                if !self.is_function {
                    eprintln!("Error: Return statement must be inside a function");
                    exit(0);
                }
                return self.parse_return_statement();
            },
            TokenType::Class => return self.parse_class_declaration(),
            TokenType::Break => {
                if !self.is_loop {
                    eprintln!("Error: Break statement must be inside a loop");
                    exit(0);
                }

                return self.parse_break_statement();
            },
            TokenType::If => return self.parse_if_statement(),
            TokenType::Else => return self.parse_if_statement(),
            TokenType::While => return self.parse_while_statement(),
            TokenType::Loop => return self.parse_loop_statement(),
            TokenType::ForEach => return self.parse_for_each_statement(),
            TokenType::For => return self.parse_for_statement(),
            TokenType::Import => return self.parse_import_statement(),
            TokenType::SemiColon => {
                self.eat();
                return Box::new(ast::NullLiteral::new());
            },
            _ => {
                let expression = self.parse_expression();
                return Box::new(ast::ExpressionStatement::new(expression));
            },
            // _ => return self.parse_expression(),
        };
    }

    fn parse_import_statement(&mut self) -> Box<dyn ast::Statement> {
        self.eat();
        let path = self.expect(TokenType::String, "Error: Expected string after import keyword".to_string()).value;
        return Box::new(ast::ImportStatement::new(path));
    }

    fn parse_for_statement(&mut self) -> Box<dyn ast::Statement> {
        self.eat();
        self.is_loop = true;

        let init = self.parse_statement();
        self.expect(TokenType::SemiColon, "Error: Expected ; after for init".to_string());
        let condition = self.parse_expression();
        self.expect(TokenType::SemiColon, "Error: Expected ; after for condition".to_string());
        let increment = self.parse_expression();

        self.expect(TokenType::LSquirly, "Error: Expected { after for increment".to_string());

        let mut body: Vec<Box<dyn ast::Statement>> = Vec::new();
        while self.at().r#type != TokenType::RSquirly {
            body.push(self.parse_statement());
        }

        self.expect(TokenType::RSquirly, "Error: Expected } after for body".to_string());

        self.is_loop = false;
        return Box::new(ast::ForStatement::new(init, condition, increment, body));
    }

    fn parse_for_each_statement(&mut self) -> Box<dyn ast::Statement> {
        self.eat();
        self.is_loop = true;

        let identifier = self.expect(TokenType::Identifier, "Error: Expected identifier after for keyword".to_string()).value;
        self.expect(TokenType::In, "Error: Expected in keyword after for identifier".to_string());
        let expression = self.parse_expression();

        self.expect(TokenType::LSquirly, "Error: Expected { after for expression".to_string());

        let mut body: Vec<Box<dyn ast::Statement>> = Vec::new();
        while self.at().r#type != TokenType::RSquirly {
            body.push(self.parse_statement());
        }

        self.expect(TokenType::RSquirly, "Error: Expected } after for body".to_string());

        self.is_loop = false;
        return Box::new(ast::ForEachStatement::new(identifier, expression, body));
    }

    fn parse_loop_statement(&mut self) -> Box<dyn ast::Statement> {
        self.eat();
        self.expect(TokenType::LSquirly, "Error: Expected { after loop keyword".to_string());

        self.is_loop = true;
        let mut body: Vec<Box<dyn ast::Statement>> = Vec::new();
        while self.at().r#type != TokenType::RSquirly {
            body.push(self.parse_statement());
        }

        self.expect(TokenType::RSquirly, "Error: Expected } after loop body".to_string());

        self.is_loop = false;
        return Box::new(ast::LoopStatement::new(body));
    }

    fn parse_while_statement(&mut self) -> Box<dyn ast::Statement> {
        self.eat();
        let condition = self.parse_expression();
        self.expect(TokenType::LSquirly, "Error: Expected { after while condition".to_string());

        self.is_loop = true;
        let mut body: Vec<Box<dyn ast::Statement>> = Vec::new();
        while self.at().r#type != TokenType::RSquirly {
            body.push(self.parse_statement());
        }

        self.expect(TokenType::RSquirly, "Error: Expected } after while body".to_string());

        self.is_loop = false;
        return Box::new(ast::WhileStatement::new(condition, body));
    }

    fn parse_if_statement(&mut self) -> Box<dyn ast::Statement> {
        self.eat();
        let condition = self.parse_expression();
        self.expect(TokenType::LSquirly, "Error: Expected { after if condition".to_string());

        let mut body: Vec<Box<dyn ast::Statement>> = Vec::new();
        while self.at().r#type != TokenType::RSquirly {
            body.push(self.parse_statement());
        }

        self.expect(TokenType::RSquirly, "Error: Expected } after if body".to_string());

        if self.at().r#type == TokenType::Else {
            self.eat();
            self.expect(TokenType::LSquirly, "Error: Expected { after else keyword".to_string());

            let mut else_body: Vec<Box<dyn ast::Statement>> = Vec::new();
            while self.at().r#type != TokenType::RSquirly {
                else_body.push(self.parse_statement());
            }

            self.expect(TokenType::RSquirly, "Error: Expected } after else body".to_string());

            return Box::new(ast::ConditionalStatement::new(condition, body, else_body));
        }

        return Box::new(ast::ConditionalStatement::new(condition, body, Vec::new()));
    }

    fn parse_break_statement(&mut self) -> Box<dyn ast::Statement> {
        self.eat();
        if self.at().r#type == TokenType::SemiColon {
            self.eat();
        }

        return Box::new(ast::BreakStatement::new());
    }

    fn parse_class_declaration(&mut self) -> Box<dyn ast::Statement> {
        self.eat();
        let name = self.expect(TokenType::Identifier, "Error: Expected class name after class keyword".to_string()).value;
        self.expect(TokenType::LSquirly, "Error: Expected { after class name".to_string());

        let mut body: Vec<Box<dyn ast::Statement>> = Vec::new();
        while self.at().r#type != TokenType::RSquirly {
            body.push(self.parse_statement());
        }

        self.expect(TokenType::RSquirly, "Error: Expected } after class declaration".to_string());

        return Box::new(ast::ClassDeclaration::new(name, body));
    }

    fn parse_return_statement(&mut self) -> Box<dyn ast::Statement> {
        self.eat();
        let expression = self.parse_expression();

        if self.at().r#type == TokenType::SemiColon {
            self.eat();
        }

        return Box::new(ast::ReturnStatement::new(expression));
    }

    fn parse_comment(&mut self) -> Box<dyn ast::Statement> {
        while self.at().r#type != TokenType::CloseComment {
            self.eat();
        }
        self.expect(TokenType::CloseComment, "Error: Expected close comment".to_string());
        return Box::new(ast::NullLiteral::new());
    }
    
    fn parse_variable_declaration(&mut self) -> Box<dyn ast::Statement> {
        let is_constant = self.eat().r#type == TokenType::Constant;
        let identifier = self.expect(TokenType::Identifier, "Error: Expected identifier".to_string()).value;

        if self.at().r#type == TokenType::SemiColon {
            self.eat();
            if is_constant {
                eprintln!("Error: Constant variable '{}' must be initialized", identifier);
                exit(0);
            }

            return Box::new(ast::VariableDeclaration::new(is_constant, identifier, Box::new(ast::NullLiteral::new())));
        }

        self.expect(TokenType::Equals, "Error: Expected assignment".to_string());
        let declaration = Box::new(ast::VariableDeclaration::new(is_constant, identifier, self.parse_expression()));

        if !self.is_loop {
            if self.at().r#type == TokenType::SemiColon {
                self.eat();
            }
        }

        return declaration;
    }

    fn parse_function_declaration(&mut self) -> Box<dyn ast::Statement> {
        self.eat();
        let mut name: String = "".to_string();

        if self.at().r#type != TokenType::OpenParen {
            name = self.expect(TokenType::Identifier, "Error: Expected function name after fn keyword".to_string()).value
        }

        let args: Vec<Box<dyn ast::Expression>> = self.parse_args();
        let mut params: Vec<String> = vec![];

        for arg in args {
            match arg.kind() {
                ast::NodeType::IdentifierType => {
                    params.push(arg.to_string());
                },
                _ => {
                    eprintln!("Error: Expected identifier");
                    exit(0);
                }
            }
        }

        self.expect(TokenType::LSquirly, "Error: Expected open curly brace".to_string());

        if !self.is_function {
            self.is_function = true;
        }

        let mut body: Vec<Box<dyn ast::Statement>> = Vec::new();
        while self.at().r#type != TokenType::EndOfFile && self.at().r#type != TokenType::RSquirly {
            body.push(self.parse_statement());
        }

        self.expect(TokenType::RSquirly, "Error: Expected close curly brace".to_string());

        self.is_function = false;
        return Box::new(ast::FunctionDeclaration::new(params, name, body, false));
    }

    fn parse_args(&mut self) -> Vec<Box<dyn ast::Expression>> {
        self.expect(TokenType::OpenParen, "Error: Expected open parenthesis".to_string());

        let args: Vec<Box<dyn ast::Expression>>;
        if self.at().r#type == TokenType::CloseParen {
            args = Vec::new();
        } else {
            args = self.parse_args_list();
        }

        self.expect(TokenType::CloseParen, "Error: Expected close parenthesis".to_string());
        return args;
    }

    fn parse_args_list(&mut self) -> Vec<Box<dyn ast::Expression>> {
        let mut args = vec![self.parse_assignment_expression()];

        while self.at().r#type == TokenType::Comma {
            self.eat();
            args.push(self.parse_assignment_expression());
        }

        return args;
    }

    fn parse_assignment_expression(&mut self) -> Box<dyn ast::Expression> {
        let left = self.parse_or_expression();

        if self.at().r#type == TokenType::Equals {
            self.eat();
            let value = self.parse_assignment_expression();

            return Box::new(ast::AssignmentExpression::new(left, value));
        }

        return left;
    }

    fn parse_or_expression(&mut self) -> Box<dyn ast::Expression> {
        let mut left = self.parse_and_expression();

        while self.at().r#type == TokenType::Or {
            self.eat();
            let right = self.parse_and_expression();

            left = Box::new(ast::LogicalExpression::new(left, right, "or".to_string()));
        }

        return left;
    }

    fn parse_and_expression(&mut self) -> Box<dyn ast::Expression> {
        let mut left = self.parse_xor_expression();

        while self.at().r#type == TokenType::And {
            self.eat();
            let right = self.parse_xor_expression();

            left = Box::new(ast::LogicalExpression::new(left, right, "and".to_string()));
        }

        return left;
    }

    fn parse_xor_expression(&mut self) -> Box<dyn ast::Expression> {
        let mut left = self.parse_not_expression();

        while self.at().r#type == TokenType::Xor {
            self.eat();
            let right = self.parse_not_expression();

            left = Box::new(ast::LogicalExpression::new(left, right, "xor".to_string()));
        }

        return left;
    }

    fn parse_not_expression(&mut self) -> Box<dyn ast::Expression> {
        if self.at().r#type == TokenType::Not {
            self.eat();
            let expression = self.parse_not_expression();

            return Box::new(ast::LogicalExpression{right: expression, operator: "not".to_string(), left: Box::new(ast::NullLiteral::new())});
        }

        return self.parse_comparison_expression();
    }

    fn parse_comparison_expression(&mut self) -> Box<dyn ast::Expression> {
        let mut left = self.parse_object_expression();

        while self.at().value == ">" || self.at().value == "<" || (self.at().value == "=" && self.peek().value == "=") || self.at().value == "!=" {
            let mut operator = self.eat().value;
            if self.at().value == "=" {
                operator += self.eat().value.as_str();
            }

            let right = self.parse_object_expression();

            left = Box::new(ast::BinaryExpression::new(left, right, operator));
        }

        return left;
    }

    fn parse_object_expression(&mut self) -> Box<dyn ast::Expression> {
        if self.at().r#type != TokenType::LSquirly {
            return self.parse_array_expression();
        }

        self.eat();
        let mut properties: Vec<ast::Property> = Vec::new();

        while self.is_not_eof() && self.at().r#type != TokenType::RSquirly {
            let key = self.expect(TokenType::Identifier, "Expected identifier as object key".to_string()).value;

            if self.at().r#type == TokenType::Comma {
                self.eat();
                properties.push(ast::Property::new(key, Box::new(ast::NullLiteral::new())));
                continue;
            } else if self.at().r#type == TokenType::RSquirly {
                properties.push(ast::Property::new(key, Box::new(ast::NullLiteral::new())));
                continue;
            }

            self.expect(TokenType::Colon, "Expected : after object key".to_string());
            let value = self.parse_expression();
            properties.push(ast::Property::new(key, value));

            if self.at().r#type != TokenType::RSquirly {
                self.expect(TokenType::Comma, "Expected , after object property".to_string());
            }
        }

        self.expect(TokenType::RSquirly, "Object literal must end with a }".to_string());
        return Box::new(ast::ObjectLiteral::new(properties));
    }

    fn parse_array_expression(&mut self) -> Box<dyn ast::Expression> {
        if self.at().r#type != TokenType::OpenBracket {
            return self.parse_bitwise();
        }
        self.eat();
        let mut elements: Vec<Box<dyn ast::Expression>> = Vec::new();
        while self.at().r#type != TokenType::CloseBracket {
            elements.push(self.parse_expression());
            if self.at().r#type == TokenType::Comma {
                self.eat();
            }
        }
        self.expect(TokenType::CloseBracket, "Expected closing bracket after array expression".to_string());
        return Box::new(ast::ArrayLiteral::new(elements));
    }

    fn parse_bitwise(&mut self) -> Box<dyn ast::Expression> {
        let mut left = self.parse_bitwise_shift_bit();

        while self.at().value == "&" || self.at().value == "|" || self.at().value == "^" {
            let operator = self.eat().value;

            let right = self.parse_bitwise_shift_bit();

            left = Box::new(ast::BinaryExpression::new(left, right, operator));
        }

        return left
    }

    fn parse_bitwise_shift_bit(&mut self) -> Box<dyn ast::Expression> {
        let mut left = self.parse_additive_expression();

        while self.at().value == "<<" || self.at().value == ">>" || self.at().value == ">>>" {
            let operator = self.eat().value;

            let right = self.parse_additive_expression();

            left = Box::new(ast::BinaryExpression::new(left, right, operator));
        }

        return left
    }

    fn parse_additive_expression(&mut self) -> Box<dyn ast::Expression> {
        let mut left = self.parse_multiplicative_expression();

        while self.at().value == "+" || self.at().value == "-" {
            let operator = self.eat().value;

            let right = self.parse_multiplicative_expression();

            left = Box::new(ast::BinaryExpression::new(left, right, operator));
        }

        return left
    }

    fn parse_multiplicative_expression(&mut self) -> Box<dyn ast::Expression> {
        let mut left = self.parse_call_member_expression();

        while self.at().value == "*" || self.at().value == "/" || self.at().value == "%" || self.at().value == "**" || self.at().value == "//" {
            let operator = self.eat().value;
            
            let right = self.parse_call_member_expression();

            left = Box::new(ast::BinaryExpression::new(left, right, operator));
        }

        return left
    }

    fn parse_call_member_expression(&mut self) -> Box<dyn ast::Expression> {
        let member = self.parse_member_expression();

        while self.at().r#type == TokenType::OpenParen {
            return self.parse_call_expression(member);
        }

        return member;
    }

    fn parse_call_expression(&mut self, member: Box<dyn ast::Expression>) -> Box<dyn ast::Expression> {
        let mut call_expression: Box<dyn ast::Expression> = Box::new(ast::CallExpression::new(member, self.parse_args()));

        if self.at().r#type == TokenType::OpenParen {
            call_expression = self.parse_call_expression(call_expression);
        }

        return call_expression;
    }

    fn parse_member_expression(&mut self) -> Box<dyn ast::Expression> {
        let mut object = self.parse_primary_expression();

        while self.at().r#type == TokenType::Dot || self.at().r#type == TokenType::OpenBracket {
            let operator = self.eat();
            let property: Box<dyn ast::Expression>;
            let computed: bool;

            if operator.r#type == TokenType::Dot {
                computed = false;
                property = self.parse_primary_expression();

                match property.kind() {
                    ast::NodeType::IdentifierType => {},
                    _ => {
                        eprintln!("Expected identifier .");
                        exit(0);
                    }
                }
            } else {
                computed = true;
                property = self.parse_expression();
                self.expect(TokenType::CloseBracket, "Expected closing bracket after computed property".to_string());
            }

            object = Box::new(ast::MemberExpression::new(object, property, computed));
        }

        return object;
    }

    fn parse_primary_expression(&mut self) -> Box<dyn ast::Expression> {
        let token = self.at().r#type;

        match token {
            TokenType::Identifier => {
                return Box::new(ast::Identifier::new(self.eat().value));
            },
            TokenType::Number => {
                return Box::new(ast::NumericLiteral::new(self.eat().value.parse::<f64>().unwrap()));
            },
            TokenType::String => {
                return Box::new(ast::StringLiteral::new(self.eat().value));
            },
            TokenType::Whitespace => {
                self.eat();
                return self.parse_primary_expression();
            },
            TokenType::OpenParen => {
                self.eat();
                let expression = self.parse_expression();
                self.expect(TokenType::CloseParen, "Expected closing parenthesis after expression".to_string());
                return expression;
            },
            TokenType::UnaryOperator => {
                let operator = self.eat().value;
                let expression = self.parse_primary_expression();
                return Box::new(ast::UnaryExpression::new(expression, operator));
            },
            _ => {
                eprintln!("Unexpected token: {:?}", token);
                exit(0);
            }
        }
    }

    fn parse_expression(&mut self) -> Box<dyn ast::Expression> {
        return self.parse_assignment_expression();
    }

    fn at(&self) -> &Token {
        return &self.tokens[0];
    }

    fn eat(&mut self) -> Token {
        return self.tokens.remove(0);
    }

    fn peek(&self) -> &Token {
        return &self.tokens[1];
    }

    fn expect(&mut self, token: TokenType, message: String) -> Token {
        if self.at().r#type != token {
            eprintln!("{}", message);
            exit(0);
        }
        return self.eat();
    }

    fn is_not_eof(&self) -> bool {
        return self.tokens[0].r#type != TokenType::EndOfFile;
    }
}
