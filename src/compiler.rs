use nom::be_i8;
use std::str;

#[derive(Debug)]
enum OpType {
    Add,
    Substract,
    Multiply,
    Divide,
}

#[derive(Debug)]
enum Expression {
    Value(i8),
    BinaryExpression(BinaryExpression),
}

#[derive(Debug)]
struct BinaryExpression {
    op: OpType,
    left: i8,
    right: i8
}

#[derive(Debug)]
struct Assignment {
    name: String,
    value: Expression
}

// named!(store(&str) -> (&str, &str, &str, i8), tuple!(
//         tag!("🍱"),
//         take!(1),
//         tag!(" "),
//         be_i8
// ));
named!(store, tag!("🍱"));
named!(assignment<&str>, do_parse!(
        map_res!(store, str::from_utf8) >>
        name: map_res!(take_until!(" "), str::from_utf8) >>
        tag!(" ") >>
        value: map_res!(tag!("3"), str::from_utf8) >>
        (name, value)
));

pub fn test() {
    let code = "🍱🐈 3";
    let res = assignment(&code.as_bytes());
    println!("{:?}", res);
}
