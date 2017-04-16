#[macro_use]
extern crate nom;

mod compiler;
mod parser;
mod vm;

fn main() {
    vm::VM::run(vec![0, 8, 0, 6, 19, 64]);

    let source = include_str!("../examples/first.🐛");
    let parsed = parser::run(source);
    println!("{:?}", parsed);

    compiler::test(parsed);
}
