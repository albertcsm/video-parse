use std::{fmt, fs::File, io};

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
}

impl fmt::Display for MdatBox {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let nalus = self.nalu_list.get_units().iter().map(|x| x.to_string()).collect::<Vec<_>>().join("\n  ");
        write!(f, "mdat({})", nalus)
    }
}
