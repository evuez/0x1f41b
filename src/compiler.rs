use nom::be_i8;

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

named!(store(&str) -> Assignment, do_parse!(
    tag!("🍱") >>
    name:take!(1)   >>
    tag!(" ")   >>
    value:be_i8   >>
    (Assignment { name: String::from(name), value: Expression::Value(value) })
));

pub fn test() {
    let code = "🍱🐈 3";
    let res = store(code);
    println!("{:?}", res);
}
