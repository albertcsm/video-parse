use std::{fmt, io::{self, Read, Seek}};
use byteorder::{BigEndian, ReadBytesExt};

use super::{nalu::Nalu};

pub struct UnknownNalu {
    nal_unit_type: u8,
    payload_size: u32
}

impl UnknownNalu {
    pub fn read(rdr: &mut (impl Read + Seek), len: u32, nal_unit_type: u8) -> io::Result<Self> {
        rdr.seek(io::SeekFrom::Current(i64::try_from(len).unwrap()));
        Ok(UnknownNalu {
            nal_unit_type,
            payload_size: len
        })
    }
}

impl Nalu for UnknownNalu {
    fn get_payload_size(&self) -> u32 {
        self.payload_size
    }
}

impl fmt::Display for UnknownNalu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[NALU {}]", self.nal_unit_type)
    }
}
