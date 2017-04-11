use nom::be_i8;
use std::str;

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

/////////////////// TEST
named!(store, tag!("🍱"));
named!(assignment<(&str, &str)>, do_parse!(
        store >>
        name: map_res!(take_until!(" "), str::from_utf8) >>
        tag!(" ") >>
        value: map_res!(tag!("3"), str::from_utf8) >>
        ((name, value))
));
/////////////////// END TEST

named!(operator<Operator>, map_res!(alt!(
    tag!("🍱") |
    tag!("🍳") |
    tag!("🏧") |
    tag!("🍇") |
    tag!("🔪") |
    tag!("🖨️")
), Operator::from_utf8));

named!(token<Element>, map_res!(take_until!(" "), Element::from_utf8));

named!(expression<(Operator, Element)>, do_parse!(
    operator: operator >>
    element: token >>
    ((operator, element))
));

// named!(expression<Expression>, do_parse!(
//     operator: operator,
//     alt!(expresssion | token)
// ));

pub fn test() {
    let code = "🍱🐈 3";
    let res = expression(&code.as_bytes());
    println!("{:?}", res);
}
