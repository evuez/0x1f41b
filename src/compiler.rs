use nom;
use nom::{multispace, space};
use std::str;
use std::vec::Vec;

#[derive(Debug)]
struct Expression {
    operator: Operator,
    elements: Vec<Element>
}

#[derive(Debug)]
enum Operator {
    Store,
    Add,
    Substract,
    Multitply,
    Divide,
    Print,
}

#[derive(Debug)]
enum Element {
    Name(String),
    Value(i8),
    Expression(Expression)
}

enum CompilerError<'a> {
    NotAnOperator(&'a str),
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

named!(operator<Operator>, map_res!(alt!(
    tag!("🍱") |
    tag!("🍳") |
    tag!("🏧") |
    tag!("🍇") |
    tag!("🔪") |
    tag!("🖨️")
), Operator::from_utf8));

named!(token<Element>, map_res!(
    terminated!(alt!(is_not_s!(" ") | is_not_s!("\n") | is_not_s!("\r")), multispace),
    Element::from_utf8
)); // make sure we can't use an operator as a token.
named!(tokens<Vec<Element> >, many0!(token));

named!(expression<Expression>, do_parse!(
    opt!(space) >>
    operator: operator >>
    elements: tokens >>
    (Expression { operator: operator, elements: elements})
));

named!(expression1<Element>, do_parse!(
    operator: operator >>
    element1: alt!(token | expression1) >> // many1!(elements)
    (Element::Expression(Expression { operator: operator, elements: vec![element1]}))
));

// named!(expression<Expression>, do_parse!(
//     operator: operator,
//     alt!(expression | token)
// ));

pub fn test() {
    let code = "🍱🐈 3 ";
    let res = expression(&code.as_bytes());
    println!("{:?}", res);
}
