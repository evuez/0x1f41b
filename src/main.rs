mod vm;

fn main() {
    println!("Hello, world!");

    vm::VM::run(vec![1]);
}
