use std::mem;

macro_rules! binary_op {
    ( $inst:ident, $op:tt ) => {{
        let a = &$inst.pop();
        let b = &$inst.pop();
        &$inst.push(a $op b);
    }}
}

const STACK_SIZE: usize = 1000;

enum Opcode {
    LITERAL,
    ADD,
    SUBSTRACT,
    MULTIPLY,
    PRINT,
}

impl Opcode {
    fn from_byte(byte: &u8) -> Option<Opcode> {
        match *byte {
            0x00 => Some(Opcode::LITERAL),
            0x01 => Some(Opcode::ADD),
            0x02 => Some(Opcode::SUBSTRACT),
            0x03 => Some(Opcode::MULTIPLY),
            0x04 => Some(Opcode::PRINT),
            _    => None
        }
    }
}

struct VM {
    stack: [i8; STACK_SIZE as usize],
    index: usize
}

impl VM {
    pub fn run(bytecode: Vec<u8>) {
        let mut vm = VM {
            stack: mem::uninitialized,
            index: 0
        };

        for (_offset, byte) in bytecode.iter().enumerate() {
            vm.execute(Opcode::from_byte(byte).unwrap());
        }
    }

    fn execute(&mut self, opcode: Opcode) {
        match opcode {
            Opcode::LITERAL => {

            },
            Opcode::ADD => { binary_op!(self, +) },
            Opcode::SUBSTRACT => { binary_op!(self, -) },
            Opcode::MULTIPLY => { binary_op!(self, *) },
            Opcode::PRINT => { println!("{}", self.pop()) },
        }
    }

    fn push(&mut self, value: i8) {
        assert!(self.index < STACK_SIZE);
        self.index += 1;
        self.stack[self.index] = value;
    }

    fn pop(&mut self) -> i8 {
        assert!(self.index > 0);
        self.index -= 1;
        self.stack[self.index]
    }
}
