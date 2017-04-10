#[macro_use]
extern crate nom;

mod compiler;
mod vm;

fn main() {
    println!("Hello, world!");

    vm::VM::run(vec![0, 8, 0, 6, 4, 5]);
    compiler::test();
}
