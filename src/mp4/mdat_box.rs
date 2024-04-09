use std::{fmt, io::{self, Read, Seek}};

use crate::h264::{nalu::Nalu, nalu_reader};

use super::atom::Atom;

pub struct MdatBox {
    pub nalus: Vec<Box<dyn Nalu>>,
    pub payload_size: u64
}

impl MdatBox {
    pub fn read(rdr: &mut (impl Read + Seek), len: u64) -> io::Result<Self> {
        let nalus = nalu_reader::read_nalus(rdr, len);
        Ok(MdatBox {
            nalus,
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
        let nalus = self.nalus.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",");
        write!(f, "mdat({})", nalus)
    }
}
