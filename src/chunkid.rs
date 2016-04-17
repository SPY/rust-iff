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

static RESERVED_CHUNK_IDS: [&'static [u8; 4]; 32] = [
    b"LIST", b"LIS1", b"LIS2", b"LIS3", b"LIS4", b"LIS5", b"LIS6", b"LIS7", b"LIS8", b"LIS9", 
    b"FORM", b"FOR1", b"FOR2", b"FOR3", b"FOR4", b"FOR5", b"FOR6", b"FOR7", b"FOR8", b"FOR9",
    b"CAT ", b"CAT1", b"CAT2", b"CAT3", b"CAT4", b"CAT5", b"CAT6", b"CAT7", b"CAT8", b"CAT9",
    b"PROP",
    b"    "
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
    
    pub fn to_str(&self) -> &str {
        str::from_utf8(&self.0).unwrap()
    }
    
    pub fn is_reserved(&self) -> bool {
        RESERVED_CHUNK_IDS.contains(&&self.0)
    }
}

impl str::FromStr for ChunkId {
    type Err = ChunkIdError;
    
    fn from_str(s: &str) -> Result {
        ChunkId::new(s.as_bytes())
    }
}

impl fmt::Display for ChunkId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}
