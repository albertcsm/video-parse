use std::{any::Any, fmt, fs::File, io::{self, Cursor, Write}};

use byteorder::{BigEndian, WriteBytesExt};

use super::{atom::Atom, h264_nalu_list::H264NaluList};

pub struct MdatBox {
    pub nalu_list: H264NaluList,
    pub payload_size: u64
}

impl MdatBox {
    pub fn read(rdr: &mut File, len: u64) -> io::Result<Self> {
        let nalu_list = H264NaluList::read(rdr, len);
        Ok(MdatBox {
            nalu_list,
            payload_size: len
        })
    }
}

impl Atom for MdatBox {
    fn get_payload_size(&self) -> u64 {
        self.payload_size
    }
    
    fn write(&self, wtr: &mut File) {
        let mut cursor = Cursor::new(Vec::new());
        self.nalu_list.write(&mut cursor);
        let buffer = cursor.into_inner();
        let total_size = 8 + buffer.len();  // total size includes the size field itself

        wtr.write_u32::<BigEndian>(total_size.try_into().unwrap()).unwrap();        
        wtr.write_all(b"mdat").unwrap();
        wtr.write_all(&buffer).unwrap();
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl fmt::Debug for MdatBox {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MdatBox")
        .field("nalu_list", &self.nalu_list)
        .finish()
    }
}
