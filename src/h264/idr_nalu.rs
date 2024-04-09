use std::{fmt, io::{self, Read, Seek}};

use super::{descriptor_reader::DescriptorReader, nalu::Nalu};

pub struct IdrNalu {
    pub first_mb_in_slice: u64,
    pub slice_type: u64,
    pub pic_parameter_set_id: u64,
    pub payload_size: u32
}

impl IdrNalu {
    pub fn read(rdr: &mut (impl Read + Seek), len: u32) -> io::Result<Self> {
        let mut descriptor_reader = DescriptorReader::new(rdr);
        let first_mb_in_slice = descriptor_reader.read_ue_v();
        let slice_type = descriptor_reader.read_ue_v();
        let pic_parameter_set_id = descriptor_reader.read_ue_v();

        let read = descriptor_reader.get_num_read_bytes();
        rdr.seek(io::SeekFrom::Current(i64::from(len) - i64::try_from(read).unwrap())).unwrap();
        Ok(IdrNalu {
            first_mb_in_slice,
            slice_type,
            pic_parameter_set_id,
            payload_size: len
        })
    }
}

impl Nalu for IdrNalu {
    fn get_payload_size(&self) -> u32 {
        self.payload_size
    }
}

impl fmt::Display for IdrNalu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[IDR(first_mb_in_slice={}, slice_type={}, pic_parameter_set_id={})]", self.first_mb_in_slice, self.slice_type, self.pic_parameter_set_id)
    }
}
