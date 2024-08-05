use crate::{
    chunk::{Chunk, OpCode},
    value::print_value,
};

pub fn dissasemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);

    let mut offset = 0;
    while offset < chunk.code.len() {
        offset = dissasemble_instruction(chunk, offset);
        offset += 1
    }
}

fn dissasemble_instruction(chunk: &Chunk, offset: usize) -> usize {
    print!("{:04} ", offset);

    let instruction =
        num::FromPrimitive::from_u8(*chunk.code.get(offset).expect("Offset out of bounds"));
    match instruction {
        Some(OpCode::OP_CONSTANT) => constant_instruction("OP_CONSTANT", offset, chunk),
        Some(OpCode::OP_RETURN) => simple_instruction("OP_RETURN", offset),
        None => {
            println!("Unknown opcode {:?}", instruction);
            offset + 1
        }
    }
}

fn constant_instruction(name: &str, offset: usize, chunk: &Chunk) -> usize {
    let constant = chunk.code.get(offset + 1).expect("Offset out of bounds");

    print!("{:-16} {:4} '", name, constant);
    print_value(chunk.constants.values.get(*constant as usize));
    print!("'\n");
    offset + 2
}

fn simple_instruction(name: &str, offset: usize) -> usize {
    println!("{name}");

    offset + 1
}
