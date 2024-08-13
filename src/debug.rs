use crate::{
    chunk::{Chunk, OpCode},
    value::print_value,
};

pub fn dissasemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);

    let mut offset = 0;
    while offset < chunk.code.len() {
        offset = dissasemble_instruction(chunk, offset);
    }
}

pub fn dissasemble_instruction(chunk: &Chunk, offset: usize) -> usize {
    print!("{:04} ", offset);
    if offset > 0 && chunk.get_line(offset) == chunk.get_line(offset - 1) {
        print!("   | ");
    } else {
        print!("{:4} ", chunk.get_line(offset))
    }

    let instruction =
        num::FromPrimitive::from_u8(*chunk.code.get(offset).expect("Offset out of bounds"));
    match instruction {
        Some(OpCode::OP_CONSTANT) => constant_instruction("OP_CONSTANT", offset, chunk),
        Some(OpCode::OP_CONSTANT_LONG) => constant_instruction("OP_CONSTANT_LONG", offset, chunk),
        Some(OpCode::OP_RETURN) => simple_instruction("OP_RETURN", offset),
        None => {
            println!("Unknown opcode {:?}", instruction);
            offset + 1
        }
    }
}

fn constant_instruction(name: &str, offset: usize, chunk: &Chunk) -> usize {
    let jmp = match name {
        "OP_CONSTANT" => 2,      // 1 byte for opcode, 1 byte for index
        "OP_CONSTANT_LONG" => 4, // 1 byte for opcode, 3 bytes for index
        _ => unreachable!(),
    };

    let index: usize = match name {
        "OP_CONSTANT" => (*chunk.code.get(offset + 1).expect("Offset out of bounds")).into(),
        "OP_CONSTANT_LONG" => {
            let index = *chunk.code.get(offset + 1).expect("Offset out of bounds") as u32
                | (*chunk.code.get(offset + 2).expect("Offset out of bounds") as u32) << 8
                | (*chunk.code.get(offset + 3).expect("Offset out of bounds") as u32) << 16;
            index.try_into().expect("u32 does not fit into usize")
        }
        _ => unreachable!(),
    };

    print!("{:-16} {:4} '", name, index);
    print_value(chunk.constants.values.get(index as usize));
    print!("'\n");
    offset + jmp
}

fn simple_instruction(name: &str, offset: usize) -> usize {
    println!("{name}");

    offset + 1
}
