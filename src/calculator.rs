#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Token {
    Number(u32),
    Op(Operator),
    Bracket(char),
}

pub struct Calculator {}

#[derive(Debug)]
pub enum Error {
    BadToken(char),
    MismatchedParen,
}

impl Calculator {
    pub fn parse<T: AsRef<str>>(expr: T) -> Result<Vec<Token>, Error> {
        let expr = expr.as_ref();
        let chars = expr.chars();
        let mut tokens: Vec<Token> = vec![];
        let mut paren = Vec::new();
        for c in chars {
            match c {
                '0'..='9' => match tokens.last_mut() {
                    Some(Token::Number(n)) => {
                        *n = *n * 10 + (c as u32 - 48);
                    }
                    _ => tokens.push(Token::Number(c as u32 - 48)),
                },
                '+' => tokens.push(Token::Op(Operator::Add)),
                '-' => tokens.push(Token::Op(Operator::Sub)),
                '/' => tokens.push(Token::Op(Operator::Div)),
                '*' => tokens.push(Token::Op(Operator::Mul)),
                '(' => {
                    tokens.push(Token::Bracket('('));
                    paren.push(c);
                }
                ')' => {
                    tokens.push(Token::Bracket(')'));
                    if let Some(p) = paren.pop() {
                        if p != '(' {
                            return Err(Error::MismatchedParen);
                        }
                    } else {
                        return Err(Error::MismatchedParen);
                    }
                }
                ' ' => {}
                '\n' => {}
                _ => return Err(Error::BadToken(c)),
            }
        }
        if !paren.is_empty() {
            return Err(Error::MismatchedParen);
        }
        Ok(tokens)
    }

    pub fn post_fix(mut toknes: Vec<Token>) -> Vec<Token> {
        toknes.reverse();

        let mut queue = Vec::new();
        let mut stack = Vec::new();

        while let Some(tk) = toknes.pop() {
            match tk {
                Token::Number(_) => queue.push(tk),
                Token::Op(_) => {
                    while !stack.is_empty() && stack[stack.len() - 1] >= tk {
                        queue.push(stack.pop().unwrap())
                    }
                    stack.push(tk);
                }
                Token::Bracket('(') => stack.push(tk),
                Token::Bracket(')') => {
                    while !stack.is_empty() && stack[stack.len() - 1] != Token::Bracket('(') {
                        queue.push(stack.pop().unwrap())
                    }
                    stack.pop();
                }
                _ => {}
            }
        }
        while let Some(element) = stack.pop() {
            queue.push(element);
        }
        queue
    }

    pub fn evaluate(mut post: Vec<Token>) -> Option<f32> {
        post.reverse();

        let mut stack: Vec<f32> = Vec::new();

        while let Some(token) = post.pop() {
            match token {
                Token::Number(num) => {
                    stack.push(num as f32);
                }
                Token::Op(Operator::Add) => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    stack.push(left + right);
                }
                Token::Op(Operator::Sub) => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    stack.push(left - right);
                }
                Token::Op(Operator::Div) => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    stack.push(left / right);
                }
                Token::Op(Operator::Mul) => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    stack.push(left * right);
                }
                _ => {}
            }
        }

        if stack.len() > 1 {
            return None;
        }
        stack.pop()
    }
}
