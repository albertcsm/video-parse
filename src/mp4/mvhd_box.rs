use std::{any::Any, fmt, fs::File, io::{self, Read, Seek, Write}};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use super::atom::Atom;

pub struct MvhdBox {
    pub version: u8,
    pub flags: [u8; 3],
    pub creation_time: u64,
    pub modification_time: u64,
    pub timescale: u32,
    pub duration: u64,
    pub remaining: Vec<u8>,
    pub payload_size: u64
}

impl MvhdBox {
    pub fn read(rdr: &mut (impl Read + Seek), len: u64) -> io::Result<Self> {
        let version = rdr.read_u8()?;
        let mut flags: [u8; 3] = [0; 3];
        rdr.read(&mut flags).unwrap();

        let mut remaining_len = len - 4;
        let creation_time: u64;
        let modification_time: u64;
        let timescale: u32;
        let duration: u64;
        if version == 1 {
            creation_time = rdr.read_u64::<BigEndian>()?;
            modification_time = rdr.read_u64::<BigEndian>()?;
            timescale = rdr.read_u32::<BigEndian>()?;
            duration = rdr.read_u64::<BigEndian>()?;
            remaining_len -= 28;
        } else {
            creation_time = rdr.read_u32::<BigEndian>()?.into();
            modification_time = rdr.read_u32::<BigEndian>()?.into();
            timescale = rdr.read_u32::<BigEndian>()?;
            duration = rdr.read_u32::<BigEndian>()?.into();
            remaining_len -= 16
        }

        let mut remaining = vec![0u8; remaining_len.try_into().unwrap()];
        rdr.read_exact(&mut remaining).unwrap();
        Ok(MvhdBox {
            version,
            flags,
            creation_time,
            modification_time,
            timescale,
            duration,
            remaining,
            payload_size: len
        })
    }
}

impl Atom for MvhdBox {
    fn get_payload_size(&self) -> u64 {
        self.payload_size
    }

    fn write(&self, wtr: &mut File) {
        let total_size = 8 + self.payload_size;
        wtr.write_u32::<BigEndian>(total_size.try_into().unwrap()).unwrap();
        wtr.write_all(b"mvhd").unwrap();
        wtr.write_u8(self.version).unwrap();
        wtr.write_all(&self.flags).unwrap();
        if self.version == 1 {
            wtr.write_u64::<BigEndian>(self.creation_time).unwrap();
            wtr.write_u64::<BigEndian>(self.modification_time).unwrap();
            wtr.write_u32::<BigEndian>(self.timescale).unwrap();
            wtr.write_u64::<BigEndian>(self.duration).unwrap();
        } else {
            wtr.write_u32::<BigEndian>(self.creation_time.try_into().unwrap()).unwrap();
            wtr.write_u32::<BigEndian>(self.modification_time.try_into().unwrap()).unwrap();
            wtr.write_u32::<BigEndian>(self.timescale).unwrap();
            wtr.write_u32::<BigEndian>(self.duration.try_into().unwrap()).unwrap();
        }
        wtr.write_all(&self.remaining).unwrap();
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl fmt::Debug for MvhdBox {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MvhdBox")
        .field("creation_time", &self.creation_time)
        .field("modification_time", &self.modification_time)
        .field("timescale", &self.timescale)
        .field("duration", &self.duration)
        .finish()
    }
}
