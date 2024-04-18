use std::{fmt, fs::File, io::{self, Write}};

use byteorder::{BigEndian, WriteBytesExt};

use crate::h264::nalu_list::NaluList;

use super::atom::Atom;

pub struct MdatBox {
    pub nalu_list: NaluList,
    pub payload_size: u64
}

impl MdatBox {
    pub fn read(rdr: &mut File, len: u64) -> io::Result<Self> {
        let nalu_list = NaluList::read(rdr, len);
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
        let total_size = 8 + self.payload_size;
        wtr.write_u32::<BigEndian>(total_size.try_into().unwrap()).unwrap();
        wtr.write_all(b"mdat").unwrap();

        // TODO: write actual NALUs
        let remaining = vec![0u8; self.payload_size.try_into().unwrap()];
        wtr.write_all(&remaining).unwrap();
    }
}

impl fmt::Display for MdatBox {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let nalus = self.nalu_list.get_units().iter().map(|x| x.to_string()).collect::<Vec<_>>().join("\n  ");
        write!(f, "mdat({})", nalus)
    }
}
