use std::{any::Any, fmt, io::{self, Read, Seek}};

use super::{descriptor_reader::DescriptorReader, nalu::Nalu};

pub struct PpsNalu {
    pub pic_parameter_set_id: u64,
    pub seq_parameter_set_id: u64,
    pub payload_size: u32
}

impl PpsNalu {
    pub fn read(rdr: &mut (impl Read + Seek), len: u32) -> io::Result<Self> {
        let mut descriptor_reader = DescriptorReader::new(rdr);
        let pic_parameter_set_id = descriptor_reader.read_ue_v();
        let seq_parameter_set_id = descriptor_reader.read_ue_v();
        
        let read = descriptor_reader.get_num_read_bytes();
        rdr.seek(io::SeekFrom::Current(i64::from(len) - i64::try_from(read).unwrap())).unwrap();
        Ok(PpsNalu {
            pic_parameter_set_id,
            seq_parameter_set_id,
            payload_size: len
        })
    }
}

impl Nalu for PpsNalu {
    fn get_payload_size(&self) -> u32 {
        self.payload_size
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl fmt::Display for PpsNalu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[PPS(pic_parameter_set_id={}, seq_parameter_set_id={})]", self.pic_parameter_set_id, self.seq_parameter_set_id)
    }
}
