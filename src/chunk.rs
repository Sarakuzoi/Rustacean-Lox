use num_derive::FromPrimitive;

use crate::value::{Value, ValueArray};

#[allow(non_camel_case_types)]
#[derive(FromPrimitive, Debug)]
pub enum OpCode {
    OP_CONSTANT,
    OP_RETURN,
}

pub struct Chunk {
    pub code: Vec<u8>, // (first byte for a new line, new line no.)
    pub constants: ValueArray,
    lines: Vec<(usize, usize)>,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            code: Vec::new(),
            constants: ValueArray::new(),
            lines: Vec::new(),
        }
    }

    pub fn write(&mut self, byte: u8, line: Option<usize>) {
        self.code.push(byte);
        match line {
            None => (),
            Some(x) => self.lines.push((self.code.len() - 1, x)),
        }
    }

    pub fn add_constant(&mut self, constant: Value) -> usize {
        self.constants.write(constant);
        self.constants.values.len() - 1
    }

    pub fn get_line(&self, offset: usize) -> usize {
        let (mut prev_byte, mut prev_line) = self.lines.first().expect("No lines entered");

        if offset == 0 {
            return prev_line;
        }

        for (first_byte, line) in self.lines.iter() {
            if prev_byte <= offset && &offset < first_byte {
                return prev_line;
            }
            prev_byte = *first_byte;
            prev_line = *line;
        }
        prev_line
    }
}
