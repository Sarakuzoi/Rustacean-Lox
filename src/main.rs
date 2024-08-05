use loxy_ferris::{
    chunk::{Chunk, OpCode},
    debug::dissasemble_chunk,
};

fn main() {
    let mut chunk = Chunk::new();
    let constant = chunk.add_constant(1.2);

    chunk.write(OpCode::OP_CONSTANT as u8, 123);
    chunk.write(
        constant
            .try_into()
            .expect("Constant table index larger than 255"),
        123,
    );

    chunk.write(OpCode::OP_RETURN as u8, 123);

    dissasemble_chunk(&chunk, "test chunk");
}
