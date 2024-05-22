use std::{any::Any, fmt, fs::File, io::{self, Read, Seek, Write}};

use byteorder::{BigEndian, WriteBytesExt};

use super::{atom::Atom, four_cc::FourCC};

pub struct UnknownBox {
    pub boxtype: FourCC,
    pub remaining: Vec<u8>,
    pub payload_size: u64
}

impl UnknownBox {
    pub fn read(rdr: &mut (impl Read + Seek), len: u64, boxtype: FourCC) -> io::Result<Self> {
        let mut remaining = vec![0u8; len.try_into().unwrap()];
        rdr.read_exact(&mut remaining).unwrap();

        Ok(UnknownBox {
            boxtype,
            remaining,
            payload_size: len
        })
    }
}

impl Atom for UnknownBox {
    fn get_payload_size(&self) -> u64 {
        self.payload_size
    }

    fn write(&self, wtr: &mut File) {
        let total_size = 8 + self.remaining.len();
        wtr.write_u32::<BigEndian>(total_size.try_into().unwrap()).unwrap();
        self.boxtype.write(wtr);
        wtr.write_all(&self.remaining).unwrap();
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl fmt::Debug for UnknownBox {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UnknownBox")
            .field("boxtype", &self.boxtype)
            .finish()
    }
}
