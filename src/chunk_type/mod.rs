use std::{
    fmt::Display,
    str::{self, FromStr},
};

#[derive(Debug, PartialEq, Eq)]
struct ChunkType {
    data: [u8; 4],
}

#[derive(Debug)]
struct ParseChunkError;

impl ChunkType {
    fn bytes(self: Self) -> [u8; 4] {
        self.data
    }

    fn is_valid(self: Self) -> bool {
        self.data.iter().filter(|s| !is_alphabet(s)).count() == 0 && self.data[2] <= 90
    }

    fn is_critical(self: Self) -> bool {
        self.data[0] <= 90
    }

    fn is_public(self: Self) -> bool {
        self.data[1] <= 90
    }

    fn is_reserved_bit_valid(self: Self) -> bool {
        self.data[2] <= 90
    }

    fn is_safe_to_copy(self: Self) -> bool {
        self.data[3] > 90
    }
}

fn is_alphabet(value: &u8) -> bool {
    let lower_a = 'a' as u8;
    let lower_z = 'z' as u8;
    let higher_a = 'A' as u8;
    let higher_z = 'Z' as u8;

    return if *value >= lower_a && *value <= lower_z {
        true
    } else if *value >= higher_a && *value <= higher_z {
        true
    } else {
        false
    };
}
impl TryFrom<[u8; 4]> for ChunkType {
    type Error = ParseChunkError;
    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        // check if value is alphabetical
        match value.into_iter().filter(|s| !is_alphabet(s)).count() {
            0 => Ok(ChunkType { data: value }),
            _ => Err(ParseChunkError {}),
        }
    }
}

impl FromStr for ChunkType {
    type Err = ParseChunkError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 4 {
            return Err(ParseChunkError {});
        }
        let mut chunks_real: [u8; 4] = [0; 4];
        let chunks = s.as_bytes();
        for (idx, c) in chunks.iter().enumerate() {
            chunks_real[idx] = *c;
        }
        match chunks_real.iter().filter(|s| !is_alphabet(s)).count() {
            0 => Ok(ChunkType { data: chunks_real }),
            _ => Err(ParseChunkError {}),
        }
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8(self.data.into()).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
