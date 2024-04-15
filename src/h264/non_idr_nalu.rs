use std::{any::Any, fmt, io::{self, Read, Seek}};

use super::{descriptor_reader::DescriptorReader, nalu::Nalu, slice_header::SliceHeader, sps_pps_provider::SpsPpsProvider};

pub struct NonIdrNalu {
    pub slice_header: SliceHeader,
    pub payload_size: u32
}

impl NonIdrNalu {
    pub fn read(rdr: &mut (impl Read + Seek), len: u32, sps_pps_provider: &impl SpsPpsProvider) -> io::Result<Self> {
        let mut descriptor_reader = DescriptorReader::new(rdr);
        let slice_header = SliceHeader::read(&mut descriptor_reader, false, sps_pps_provider);

        let read = descriptor_reader.get_num_read_bytes();
        rdr.seek(io::SeekFrom::Current(i64::from(len) - i64::try_from(read).unwrap())).unwrap();
        Ok(NonIdrNalu {
            slice_header,
            payload_size: len
        })
    }
}

impl Nalu for NonIdrNalu {
    fn get_payload_size(&self) -> u32 {
        self.payload_size
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl fmt::Display for NonIdrNalu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let slice_type = match self.slice_header.slice_type {
            2 | 7 => "I",
            0 | 5 => "P",
            1 | 6 => "B",
            _ => "?"
        };
        write!(f, "[NON-IDR(slice_type={}({}), frame_num={}, pic_order_cnt_lsb={})]", self.slice_header.slice_type, slice_type, self.slice_header.frame_num, self.slice_header.pic_order_cnt_lsb)
    }
}
