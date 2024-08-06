use loxy_ferris::{
    chunk::{Chunk, OpCode},
    debug::dissasemble_chunk,
};

fn main() {
    let mut chunk = Chunk::new();
    let constant = chunk.add_constant(1.2);

    chunk.write(OpCode::OP_CONSTANT as u8, Some(123));
    chunk.write(
        constant
            .try_into()
            .expect("Constant table index larger than 255"),
        None,
    );

    chunk.write(OpCode::OP_RETURN as u8, None);

    chunk.write(OpCode::OP_RETURN as u8, Some(124));
    chunk.write(OpCode::OP_CONSTANT as u8, None);
    chunk.write(
        constant
            .try_into()
            .expect("Constant table index larger than 255"),
        None,
    );

    dissasemble_chunk(&chunk, "test chunk");
}
