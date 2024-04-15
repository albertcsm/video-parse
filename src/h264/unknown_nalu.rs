use std::{any::Any, fmt, io::{self, Read, Seek}};

use super::nalu::Nalu;

pub struct UnknownNalu {
    pub nal_unit_type: u8,
    pub payload_size: u32
}

impl UnknownNalu {
    pub fn read(rdr: &mut (impl Read + Seek), len: u32, nal_unit_type: u8) -> io::Result<Self> {
        rdr.seek(io::SeekFrom::Current(i64::try_from(len).unwrap())).unwrap();
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

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl fmt::Display for UnknownNalu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[NALU {}]", self.nal_unit_type)
    }
}
