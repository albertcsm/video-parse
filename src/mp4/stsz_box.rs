use std::{any::Any, fmt, fs::File, io::{self, Read, Write}};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use super::atom::Atom;

pub struct StszBox {
    pub version: u8,
    pub flags: [u8; 3],
    pub sample_size: u32,
    pub sample_count: u32,
    pub entry_sizes: Vec<u32>,
    pub payload_size: u64
}

impl StszBox {
    pub fn read(rdr: &mut File, len: u64) -> io::Result<Self> {
        let version = rdr.read_u8()?;
        let mut flags: [u8; 3] = [0; 3];
        rdr.read(&mut flags).unwrap();

        let sample_size = rdr.read_u32::<BigEndian>().unwrap();
        let sample_count = rdr.read_u32::<BigEndian>().unwrap();
        let mut entry_sizes = vec![];
        for _i in 0..sample_count {
            let entry_size = rdr.read_u32::<BigEndian>().unwrap();
            entry_sizes.push(entry_size);
        }
        // recompute size from mdat to sample unit
        // entry_sizes[0] = 429;
        
        Ok(StszBox {
            version,
            flags,
            sample_size,
            sample_count,
            entry_sizes,
            payload_size: len
        })
    }
}

impl Atom for StszBox {
    fn get_payload_size(&self) -> u64 {
        self.payload_size
    }

    fn write(&self, wtr: &mut File) {
        let total_size = 8 + self.payload_size;
        wtr.write_u32::<BigEndian>(total_size.try_into().unwrap()).unwrap();
        wtr.write_all(b"stsz").unwrap();
        wtr.write_u8(self.version).unwrap();
        wtr.write_all(&self.flags).unwrap();

        wtr.write_u32::<BigEndian>(self.sample_size.try_into().unwrap()).unwrap();
        wtr.write_u32::<BigEndian>(self.sample_count.try_into().unwrap()).unwrap();
        for sample_size in &self.entry_sizes {
            wtr.write_u32::<BigEndian>(*sample_size).unwrap();
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl fmt::Debug for StszBox {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StszBox")
            .field("sample_size", &self.sample_size)
            .field("sample_count", &self.sample_count)
            .field("entry_sizes", &self.entry_sizes)
            .finish()
    }
}
