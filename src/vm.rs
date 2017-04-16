macro_rules! binary_op {
    ( $inst:ident, $op:tt ) => {{
        let a = &$inst.pop();
        let b = &$inst.pop();
        &$inst.push(b $op a);
    }}
}

const STACK_SIZE: usize = 1000;

enum Opcode {
    LITERAL,
    ADD,
    SUBSTRACT,
    MULTIPLY,
    DIVIDE,
    PRINT,
}

impl Opcode {
    fn from_byte(byte: &u8) -> Option<Opcode> {
        match *byte {
            0x00 => Some(Opcode::LITERAL),
            0x10 => Some(Opcode::ADD),
            0x11 => Some(Opcode::SUBSTRACT),
            0x12 => Some(Opcode::MULTIPLY),
            0x13 => Some(Opcode::DIVIDE),
            0x40 => Some(Opcode::PRINT),
            _    => None
        }
    }
}

pub struct VM {
    bytecode: Vec<u8>,
    index: usize,
    stack: [Option<i8>; STACK_SIZE],
    stack_size: usize
}

impl VM {
    pub fn run(bytecode: Vec<u8>) {
        let mut vm = VM {
            bytecode: bytecode.clone(),
            index: 0,
            stack: [None; STACK_SIZE],
            stack_size: 0
        };

        while vm.index < bytecode.len() {
            let opcode = vm.read();
            vm.execute(opcode);
            vm.next();
        }
    }

    fn execute(&mut self, opcode: Opcode) {
        match opcode {
            Opcode::LITERAL => {
                let literal = self.next().read_literal();
                self.push(literal);
            },
            Opcode::ADD => { binary_op!(self, +) },
            Opcode::SUBSTRACT => { binary_op!(self, -) },
            Opcode::MULTIPLY => { binary_op!(self, *) },
            Opcode::DIVIDE => { binary_op!(self, /) },
            Opcode::PRINT => { println!("{}", self.pop()) },
        }
    }

    fn read(&self) -> Opcode {
        Opcode::from_byte(&self.bytecode[self.index]).unwrap()
    }
    fn read_literal(&self) -> i8 {
        self.bytecode[self.index] as i8
    }

    fn next(&mut self) -> &Self {
        self.index += 1;
        self
    }

    fn push(&mut self, value: i8) {
        assert!(self.stack_size < STACK_SIZE);
        self.stack[self.stack_size] = Some(value);
        self.stack_size += 1;
    }

    fn pop(&mut self) -> i8 {
        assert!(self.stack_size > 0);
        self.stack_size -= 1;
        let value = self.stack[self.stack_size].unwrap();
        self.stack[self.stack_size] = None;
        value
    }
}
