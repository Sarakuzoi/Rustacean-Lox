use loxy_ferris::{
    chunk::{Chunk, OpCode},
    debug::dissasemble_chunk,
};

fn main() {
    let mut chunk = Chunk::new();
    chunk.write(OpCode::OP_RETURN as u8);

    dissasemble_chunk(&chunk, "test chunk");
}
