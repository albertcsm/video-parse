use std::{fmt, io::{self, Read, Seek}};
use byteorder::{BigEndian, ReadBytesExt};

use super::atom::Atom;

pub struct MvhdBox {
    pub version: u8,
    pub flags: [u8; 3],
    pub creation_time: u64,
    pub modification_time: u64,
    pub timescale: u32,
    pub duration: u64,
    pub payload_size: u64
}

impl MvhdBox {
    pub fn read(rdr: &mut (impl Read + Seek), len: u64) -> io::Result<Self> {
        let version = rdr.read_u8()?;
        let mut flags: [u8; 3] = [0; 3];
        rdr.read(&mut flags).unwrap();

        let mut remaining = len - 4;
        let creation_time: u64;
        let modification_time: u64;
        let timescale: u32;
        let duration: u64;
        if version == 1 {
            creation_time = rdr.read_u64::<BigEndian>()?;
            modification_time = rdr.read_u64::<BigEndian>()?;
            timescale = rdr.read_u32::<BigEndian>()?;
            duration = rdr.read_u64::<BigEndian>()?;
            remaining -= 28;
        } else {
            creation_time = rdr.read_u32::<BigEndian>()?.into();
            modification_time = rdr.read_u32::<BigEndian>()?.into();
            timescale = rdr.read_u32::<BigEndian>()?;
            duration = rdr.read_u32::<BigEndian>()?.into();
            remaining -= 16
        }

        rdr.seek(io::SeekFrom::Current(remaining.try_into().unwrap())).unwrap();
        Ok(MvhdBox {
            version,
            flags,
            creation_time,
            modification_time,
            timescale,
            duration,
            payload_size: len
        })
    }
}

impl Atom for MvhdBox {
    fn get_payload_size(&self) -> u64 {
        self.payload_size
    }
}

impl fmt::Display for MvhdBox {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "mvhd(timescale={}, duration={})", self.timescale, self.duration)
    }
}
