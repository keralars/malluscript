use crate::lexer::tokens::TokenType;

#[derive(Debug, Clone, PartialEq)]
pub struct SourceUnit(pub Vec<SourceUnitPart>);

#[derive(Debug, Clone, PartialEq)]
pub enum SourceUnitPart {
    Statement(Statement),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Conditional((usize, usize), Expression, SourceUnit, Option<SourceUnit>),
    Loop((usize, usize), Expression, SourceUnit),
    Declaration((usize, usize), Expression),
    Assignment((usize, usize), Expression, Expression),
    Write((usize, usize), Expression),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    //    Empty,
    Add((usize, usize), Box<Expression>, Box<Expression>),
    Subtract((usize, usize), Box<Expression>, Box<Expression>),
    Multiply((usize, usize), Box<Expression>, Box<Expression>),
    Divide((usize, usize), Box<Expression>, Box<Expression>),
    Modulo((usize, usize), Box<Expression>, Box<Expression>),

    UnaryMinus((usize, usize), Box<Expression>),
    Equals((usize, usize), Box<Expression>, Box<Expression>),
    GreaterThan((usize, usize), Box<Expression>, Box<Expression>),
    LessThan((usize, usize), Box<Expression>, Box<Expression>),
    NotEquals((usize, usize), Box<Expression>, Box<Expression>),
    Integer((usize, usize), TokenType),
    Symbol((usize, usize), TokenType),
    StringLiteral((usize, usize), TokenType),
    InputString((usize, usize)),
    InputNumber((usize, usize)),
}
