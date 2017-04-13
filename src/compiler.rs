use nom;
use std::str;
use std::vec::Vec;

#[derive(Debug)]
struct Expression {
    indent: u8,
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

impl Expression {
    fn guess_indent(indent: Option<&[u8]>) -> Result<u8, CompilerError> {
        match indent {
            Some(a) => Ok(a.len() as u8),
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

named!(indent<u8>, map_res!(opt!(preceded!(nom::eol, nom::space)), Expression::guess_indent));

named!(operator<Operator>, map_res!(alt!(
    tag!("🍱") |
    tag!("🍳") |
    tag!("🏧") |
    tag!("🍇") |
    tag!("🔪") |
    tag!("🖨️")
), Operator::from_utf8));

fn token(input: &[u8]) -> nom::IResult<&[u8], Element> {
    map_res!(
        input,
        do_parse!(opt!(nom::space) >> token:take_while!(call!(|c| c != '\n' as u8 && c != ' ' as u8)) >> (token)),
        Element::from_utf8
    )
}// make sure we can't use an operator as a token.

fn elements(input: &[u8], parent_indent: u8) -> nom::IResult<&[u8], Vec<Element>> {
    many1!(input, alt!(apply!(expression, parent_indent) | token))
}

fn expression(input: &[u8], parent_indent: u8) -> nom::IResult<&[u8], Element> {
    do_parse!(input,
        p_indent: indent >>
        operator: operator >>
        n_indent: indent >>// MUST BE PRECEDED BY \n, otherwise it's not an indent. Get nested indent; if >= indent keep going otherwise ERR
        elements: apply!(elements, p_indent) >>
        (Element::Expression(Expression { indent: p_indent, operator: operator, elements: elements}))
     )
}

pub fn test() {
    println!("\n\n");

    let code = &"🍱🐈 3\n 🍇4 3".as_bytes();
    //let code = &" 🍱🐈 3\n       🍇4 3".as_bytes();

    println!("{:?}", expression(code, 0));
    //println!("{:?}", indent(code));
}
