use std::{fmt, io::{self, Read, Seek}};
use byteorder::{BigEndian, ReadBytesExt};

use crate::{atom::Atom, four_cc::FourCC};

pub struct UnknownBox {
    boxtype: FourCC,
    payload_size: u64
}

impl UnknownBox {
    pub fn read(rdr: &mut (impl Read + Seek), len: u64, boxtype: FourCC) -> io::Result<Self> {
        rdr.seek(io::SeekFrom::Current(i64::try_from(len).unwrap()));
        Ok(UnknownBox {
            boxtype,
            payload_size: len
        })
    }
}

impl Atom for UnknownBox {
    fn get_payload_size(&self) -> u64 {
        self.payload_size
    }
}

impl fmt::Display for UnknownBox {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}(unknown box)", self.boxtype)
    }
}
