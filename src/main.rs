#[macro_use]
extern crate nom;

mod compiler;
mod vm;

fn main() {
    vm::VM::run(vec![0, 8, 0, 6, 19, 64]);

    compiler::test();
}
