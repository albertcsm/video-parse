use std::{fmt, io::{self, Read, Seek}};

use super::nalu::Nalu;

pub struct SeiNalu {
    pub payload_size: u32
}

impl SeiNalu {
    pub fn read(rdr: &mut (impl Read + Seek), len: u32) -> io::Result<Self> {
        rdr.seek(io::SeekFrom::Current(i64::try_from(len).unwrap())).unwrap();
        Ok(SeiNalu {
            payload_size: len
        })
    }
}

impl Nalu for SeiNalu {
    fn get_payload_size(&self) -> u32 {
        self.payload_size
    }
}

impl fmt::Display for SeiNalu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[SEI]")
    }
}
