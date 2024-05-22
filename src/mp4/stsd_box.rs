use std::{any::Any, fmt, fs::File, io::{self, Read, Write}};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use super::{atom::Atom, box_list::BoxList};

pub struct StsdBox {
    pub version: u8,
    pub flags: [u8; 3],
    pub box_list: BoxList,
    pub payload_size: u64
}

impl StsdBox {
    pub fn read(rdr: &mut File, len: u64) -> io::Result<Self> {
        let version = rdr.read_u8()?;
        let mut flags: [u8; 3] = [0; 3];
        rdr.read(&mut flags).unwrap();

        let _entry_count = rdr.read_u32::<BigEndian>()?;
        let box_list = BoxList::read(rdr, len - 8);
        Ok(StsdBox {
            version,
            flags,
            box_list,
            payload_size: len
        })
    }
}

impl Atom for StsdBox {
    fn get_payload_size(&self) -> u64 {
        self.payload_size
    }

    fn write(&self, wtr: &mut File) {
        let total_size = 8 + self.payload_size;
        wtr.write_u32::<BigEndian>(total_size.try_into().unwrap()).unwrap();
        wtr.write_all(b"stsd").unwrap();
        wtr.write_u8(self.version).unwrap();
        wtr.write_all(&self.flags).unwrap();
        wtr.write_u32::<BigEndian>(self.box_list.boxes.len().try_into().unwrap()).unwrap();
        self.box_list.write(wtr);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl fmt::Debug for StsdBox {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StsdBox")
        .field("box_list", &self.box_list)
        .finish()
    }
}
