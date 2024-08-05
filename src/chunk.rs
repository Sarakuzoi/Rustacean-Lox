use num_derive::FromPrimitive;

#[allow(non_camel_case_types)]
#[derive(FromPrimitive, Debug)]
pub enum OpCode {
    OP_RETURN,
}

pub struct Chunk {
    pub code: Vec<u8>, // or u8
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk { code: Vec::new() }
    }

    pub fn write(&mut self, byte: u8) {
        self.code.push(byte);
    }
}
