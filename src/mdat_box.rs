use std::{fmt, io::{self, Read, Seek}};
use byteorder::{BigEndian, ReadBytesExt};

use crate::{atom::Atom, four_cc::FourCC};

pub struct MdatBox {
    payload_size: u64
}

impl MdatBox {
    pub fn read(rdr: &mut (impl Read + Seek), len: u64) -> io::Result<Self> {
        rdr.seek(io::SeekFrom::Current(i64::try_from(len).unwrap()));
        Ok(MdatBox {
            payload_size: len
        })
    }
}

impl Atom for MdatBox {
    fn get_payload_size(&self) -> u64 {
        self.payload_size
    }
}

impl fmt::Display for MdatBox {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "mdat()")
    }
}
