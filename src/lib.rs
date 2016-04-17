pub mod iff {
    use std::fmt;
    
    pub mod chunkid {
        use std::fmt;
        use std::str;
        use std::result;
        
        #[derive(Debug, PartialEq, Eq)]
        pub struct ChunkId([u8; 4]);
        
        #[derive(Debug, PartialEq, Eq)]
        pub enum ChunkIdError {
            ShortLength,
            UnsupportedChar,
            SpacePrecedeLetter
        }
        
        impl fmt::Display for ChunkIdError {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match *self {
                    ChunkIdError::ShortLength => {
                        write!(f, "ChunkId source should be at least 4 bytes")
                    },
                    ChunkIdError::UnsupportedChar => {
                        write!(f, "ChunkId can contain only displayable ASCII characters")
                    },
                    ChunkIdError::SpacePrecedeLetter => {
                        write!(f, "Space cannot precede letter in ChunkId")
                    }
                }
            }
        }
        
        pub type Result = result::Result<ChunkId, ChunkIdError>;
        
        pub const LOWER_CHAR_RANGE: u8 = 0x20;
        pub const UPPER_CHAR_RANGE: u8 = 0x7E;
        pub const SPACE_CHAR_CODE: u8 = 0x20;
        
        fn is_allowed_char(chr: &u8) -> bool {
            *chr >= LOWER_CHAR_RANGE && *chr <= UPPER_CHAR_RANGE
        }
        
        fn has_precede_spaces(id: &[u8]) -> bool {
            for idx in 0..3 {
                if id[idx] == SPACE_CHAR_CODE && id[idx + 1] != SPACE_CHAR_CODE {
                    return true
                }
            }
            false
        }
        
        static RESERVED_CHUNK_IDS: [&'static str; 32] = [
            "LIST", "LIS1", "LIS2", "LIS3", "LIS4", "LIS5", "LIS6", "LIS7", "LIS8", "LIS9", 
            "FORM", "FOR1", "FOR2", "FOR3", "FOR4", "FOR5", "FOR6", "FOR7", "FOR8", "FOR9",
            "CAT ", "CAT1", "CAT2", "CAT3", "CAT4", "CAT5", "CAT6", "CAT7", "CAT8", "CAT9",
            "PROP",
            "    "
        ];
        
        impl ChunkId {
            pub fn new(slice: &[u8]) -> Result {
                if slice.len() < 4 {
                    return Err(ChunkIdError::ShortLength)
                }
                if !slice[0..4].iter().all(is_allowed_char) {
                    return Err(ChunkIdError::UnsupportedChar)
                }
                if has_precede_spaces(slice) {
                    return Err(ChunkIdError::SpacePrecedeLetter)
                }
                Ok(ChunkId([slice[0], slice[1], slice[2], slice[3]]))
            }
            
            pub fn from_str(str: &str) -> Result {
                ChunkId::new(str.as_bytes())
            }
            
            pub fn to_str(&self) -> &str {
                str::from_utf8(&self.0[0..]).unwrap()
            }
            
            pub fn is_reserved(&self) -> bool {
                RESERVED_CHUNK_IDS.contains(&self.to_str())
            }
        }
        
        impl fmt::Display for ChunkId {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}", self.to_str())
            }
        }
    }
    
    pub use self::chunkid::ChunkId;
    
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
}

#[cfg(test)]
mod test {
    use iff::*;
    use iff::chunkid::ChunkIdError;
    
    const NULL: &'static [u8] = &[0; 0];
    
    #[test]
    fn space_for_data_is_allocated() {
        let data = [0; 4];
        let chunk = Chunk::new(ChunkId::from_str("data").unwrap(), 4, &data).unwrap();
        assert!(chunk.len() == 4)
    }
    
    #[test]
    fn not_enough_data() {
        let data = [0; 4];
        let chunk = Chunk::new(ChunkId::from_str("data").unwrap(), 8, &data);
        assert!(chunk.is_none())
    }
    
    #[test]
    fn chunk_is_displayed_correct() {
        let chunk = Chunk::new(ChunkId::from_str("data").unwrap(), 0, NULL).unwrap();
        assert!(format!("{}", chunk) == r#"Chunk "data". Size 0 bytes"#)
    }
    
    #[test]
    fn chunk_id_is_unprintable() {
        let id = ChunkId::new(&[0, 1, 2, 3][0..]);
        assert!(id.unwrap_err() == ChunkIdError::UnsupportedChar)
    }
    
    #[test]
    fn short_input_for_chunk_id() {
        let id = ChunkId::new("abc".as_bytes());
        assert!(id.unwrap_err() == ChunkIdError::ShortLength)
    }
    
    #[test]
    fn chunk_id_cannot_have_inner_space() {
        assert!(ChunkId::new(" abc".as_bytes()).unwrap_err() == ChunkIdError::SpacePrecedeLetter);
        assert!(ChunkId::new("a bc".as_bytes()).unwrap_err() == ChunkIdError::SpacePrecedeLetter);
        assert!(ChunkId::new("ab c".as_bytes()).unwrap_err() == ChunkIdError::SpacePrecedeLetter);
        assert!(ChunkId::new("  ab".as_bytes()).unwrap_err() == ChunkIdError::SpacePrecedeLetter);
        assert!(ChunkId::new("a  b".as_bytes()).unwrap_err() == ChunkIdError::SpacePrecedeLetter);
        assert!(ChunkId::new("   a".as_bytes()).unwrap_err() == ChunkIdError::SpacePrecedeLetter)
    }
    
    #[test]
    fn chunk_id_can_have_trailing_spaces() {
        assert!(ChunkId::new("abc ".as_bytes()).is_ok())
    }
    
    #[test]
    fn long_input_for_chunk() {
        let id = ChunkId::new("abcde".as_bytes()).unwrap();
        assert!(id.to_str() == "abcd")
    }
    
    #[test]
    fn reserved_chunks() {
        assert!(!ChunkId::from_str("FOR0").unwrap().is_reserved());
        assert!(ChunkId::from_str("FORM").unwrap().is_reserved());
        assert!(ChunkId::from_str("    ").unwrap().is_reserved())
    }
}
