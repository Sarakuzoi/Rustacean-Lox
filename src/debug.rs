use crate::chunk::{Chunk, OpCode};

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
        Some(OpCode::OP_RETURN) => simple_instruction("OP_RETURN", offset),
        _ => {
            println!("Unknown opcode {:?}", instruction);
            offset + 1
        }
    }
}

fn simple_instruction(name: &str, offset: usize) -> usize {
    println!("{name}");

    offset + 1
}
