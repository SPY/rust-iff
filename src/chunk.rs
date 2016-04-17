use std::fmt;
pub use chunkid::ChunkId;

#[derive(Debug)]
pub struct Chunk<'a> {
    id: ChunkId,
    size: i32,
    data: &'a [u8]
}

impl <'a> Chunk<'a> {
    pub fn new(id: ChunkId, size: i32, data: &'a [u8]) -> Option<Chunk<'a>> {
        if size as usize > data.len() {
            return None
        }
        Some(Chunk { id: id, size: size, data: data })
    }
    
    pub fn len(&self) -> i32 {
        self.size
    }
}

impl <'a> fmt::Display for Chunk<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            r#"Chunk "{id}". Size {size} bytes"#,
            id = self.id.to_str(),
            size = self.size
        )
    }
}
