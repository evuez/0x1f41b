use std::str;
use std::vec::Vec;
use nom::{ErrorKind, IResult, eol, space};

const ERR_WRONG_INDENT: u32 = 0x01;

#[derive(Debug)]
pub struct Expression {
    indent: i8,
    pub operator: Operator,
    elements: Vec<Element>
}

#[derive(Debug)]
pub enum Operator {
    Store,
    Add,
    Substract,
    Multitply,
    Divide,
    Print,
}

#[derive(Debug)]
pub enum Element {
    Name(String),
    Value(i8),
    Expression(Expression)
}

enum CompilerError<'a> {
    NotAnOperator(&'a str),
}

impl Expression {
    fn guess_indent(indent: Option<&[u8]>) -> Result<i8, CompilerError> {
        match indent {
            Some(a) => Ok(a.len() as i8),
            None    => Ok(0)
        }
    }
}

impl Operator {
    fn from_utf8(v: &[u8]) -> Result<Operator, CompilerError> {
        match str::from_utf8(v).unwrap() {
            "🍱" => Ok(Operator::Store),
            "🍳" => Ok(Operator::Add),
            "🏧" => Ok(Operator::Substract),
            "🍇" => Ok(Operator::Multitply),
            "🔪" => Ok(Operator::Divide),
            "🖨️"  => Ok(Operator::Print),
            r => Err(CompilerError::NotAnOperator(r)),
        }
    }
}

impl Element {
    fn from_utf8(v: &[u8]) -> Result<Element, CompilerError> {
        let v = str::from_utf8(v).unwrap();

        match v.parse::<i8>() {
            Ok(n)  => Ok(Element::Value(n)),
            Err(_) => Ok(Element::Name(v.to_string()))
        }
    }
}

pub fn run(source: &str) -> Vec<Element> {
    let (_, r) = expressions(&source.trim().as_bytes()).unwrap();
    r
}

// Parsers

fn indent(input: &[u8], parent_indent: i8) -> IResult<&[u8], i8> {
    let indent_result = map_res!(input, opt!(preceded!(eol, space)), Expression::guess_indent);
    let (_, current_indent) = indent_result.clone().unwrap();

    match current_indent - 1 {
        c if c == parent_indent => indent_result,
        c if c > parent_indent  => panic!("Indentation isn't consistent"),
        _ => IResult::Error(ErrorKind::Custom(ERR_WRONG_INDENT))
    }
}

named!(operator<Operator>, map_res!(alt!(
    tag!("🍱") |
    tag!("🍳") |
    tag!("🏧") |
    tag!("🍇") |
    tag!("🔪") |
    tag!("🖨️")
), Operator::from_utf8));

fn token(input: &[u8]) -> IResult<&[u8], Element> {
    map_res!(
        input,
        do_parse!(opt!(space) >> token:take_while!(call!(|c| c != '\n' as u8 && c != ' ' as u8)) >> (token)),
        Element::from_utf8
    )
}

fn elements(input: &[u8], parent_indent: i8) -> IResult<&[u8], Vec<Element>> {
    many1!(input, alt!(apply!(expression, parent_indent) | token))
}

fn expression(input: &[u8], parent_indent: i8) -> IResult<&[u8], Element> {
    do_parse!(input,
        current_indent: apply!(indent, parent_indent) >>
        operator: operator >>
        elements: apply!(elements, current_indent) >>
        (Element::Expression(Expression { indent: current_indent, operator: operator, elements: elements}))
     )
}

named!(expressions<Vec<Element> >, many1!(do_parse!(opt!(eol) >> exp: apply!(expression, -1) >> (exp))));
