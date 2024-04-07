use std::{fmt, io::{self, Read, Seek}};
use byteorder::{BigEndian, ReadBytesExt};

use crate::{atom::Atom, atom_reader, four_cc::FourCC};

pub struct MoovBox {
    children: Vec<Box<dyn Atom>>,
    payload_size: u64
}

impl MoovBox {
    pub fn read(rdr: &mut (impl Read + Seek), len: u64) -> io::Result<Self> {
        // rdr.seek(io::SeekFrom::Current(i64::try_from(len).unwrap()));
        let children = atom_reader::read_atoms(rdr, len);
        Ok(MoovBox {
            children: children,
            payload_size: len
        })
    }
}

impl Atom for MoovBox {
    fn get_payload_size(&self) -> u64 {
        self.payload_size
    }
}

impl fmt::Display for MoovBox {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let children = self.children.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",");
        write!(f, "moov({})", children)
    }
}
