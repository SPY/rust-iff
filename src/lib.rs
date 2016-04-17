pub mod chunkid;
pub mod chunk;

#[cfg(test)]
mod test {
    use chunk::*;
    use chunkid::ChunkIdError;
    use std::str::FromStr;
    
    const NULL: &'static [u8] = &[0; 0];
    
    #[test]
    fn space_for_data_is_allocated() {
        let data = [0; 4];
        assert!(Chunk::new(ChunkId::from_str("data").unwrap(), 4, &data).unwrap().len() == 4);
        assert!(Chunk::new(ChunkId::from_str("data").unwrap(), 8, &data).is_none())
    }
    
    #[test]
    fn chunk_is_displayed_correct() {
        let chunk = Chunk::new(ChunkId::from_str("data").unwrap(), 0, NULL).unwrap();
        assert!(format!("{}", chunk) == r#"Chunk "data". Size 0 bytes"#)
    }
    
    #[test]
    fn chunk_id_is_unprintable() {
        let id = ChunkId::new(&[0, 1, 2, 3]);
        assert!(id.unwrap_err() == ChunkIdError::UnsupportedChar)
    }
    
    #[test]
    fn short_input_for_chunk_id() {
        assert!(ChunkId::new(b"abc").unwrap_err() == ChunkIdError::ShortLength);
        assert!(ChunkId::new(b"abcde").unwrap().to_str() == "abcd")
    }
    
    #[test]
    fn chunk_id_cannot_have_inner_space() {
        let bad_names = [" abc", "a bc", "ab c", "  ab", "a  b", "   a"];
        for id in bad_names.iter() {
            assert!(ChunkId::from_str(id).unwrap_err() == ChunkIdError::SpacePrecedeLetter)
        }
        assert!(ChunkId::new(b"abc ").is_ok())
    } 
    
    #[test]
    fn reserved_chunks() {
        assert!(!ChunkId::from_str("FOR0").unwrap().is_reserved());
        assert!(ChunkId::from_str("FORM").unwrap().is_reserved());
        assert!(ChunkId::from_str("    ").unwrap().is_reserved())
    }
}
