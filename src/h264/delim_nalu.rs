use std::{fmt, io::{self, Read, Seek}};

use super::nalu::Nalu;

pub struct DelimNalu {
    pub payload_size: u32
}

impl DelimNalu {
    pub fn read(rdr: &mut (impl Read + Seek), len: u32) -> io::Result<Self> {
        rdr.seek(io::SeekFrom::Current(i64::try_from(len).unwrap())).unwrap();
        Ok(DelimNalu {
            payload_size: len
        })
    }
}

impl Nalu for DelimNalu {
    fn get_payload_size(&self) -> u32 {
        self.payload_size
    }
}

impl fmt::Display for DelimNalu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[DELIM]")
    }
}
