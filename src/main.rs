use loxy_ferris::{
    chunk::{Chunk, OpCode},
    debug::dissasemble_chunk,
};

fn main() {
    let mut chunk = Chunk::new();

    chunk.write_constant(1.2, Some(123));
    for _ in 0..300 {
        chunk.write_constant(1.2, Some(123));
    }

    chunk.write(OpCode::OP_RETURN as u8, None);

    chunk.write(OpCode::OP_RETURN as u8, Some(124));
    chunk.write_constant(2.3, Some(124));

    dissasemble_chunk(&chunk, "test chunk");
}
