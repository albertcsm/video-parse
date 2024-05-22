use std::{any::Any, fmt, fs::File, io::{self, Write}};

use byteorder::{BigEndian, WriteBytesExt};

use super::{atom::Atom, box_list::BoxList};

pub struct TrakBox {
    pub box_list: BoxList,
    pub payload_size: u64
}

impl TrakBox {
    pub fn read(rdr: &mut File, len: u64) -> io::Result<Self> {
        let box_list = BoxList::read(rdr, len);
        Ok(TrakBox {
            box_list,
            payload_size: len
        })
    }
}

impl Atom for TrakBox {
    fn get_payload_size(&self) -> u64 {
        self.payload_size
    }

    fn write(&self, wtr: &mut File) {
        let total_size = 8 + self.payload_size;
        wtr.write_u32::<BigEndian>(total_size.try_into().unwrap()).unwrap();
        wtr.write_all(b"trak").unwrap();
        self.box_list.write(wtr);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl fmt::Debug for TrakBox {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TrakBox")
            .field("box_list", &self.box_list)
            .finish()
    }
}
