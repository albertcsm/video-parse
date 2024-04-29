use std::{any::Any, fmt, io::{self, Read, Seek, Write}};

use super::{descriptor_reader::DescriptorReader, descriptor_writer::DescriptorWriter, nalu::Nalu, slice_header::SliceHeader, sps_pps_provider::SpsPpsProvider};

pub struct IdrNalu {
    pub slice_header: SliceHeader,
    residue: (u8, u8),
    remaining: Vec<u8>,
    pub payload_size: u32
}

impl IdrNalu {
    pub fn read<'a>(rdr: &mut (impl Read + Seek), len: u32, sps_pps_provider: &impl SpsPpsProvider) -> io::Result<Self> {
        let mut descriptor_reader = DescriptorReader::new(rdr);
        let slice_header: SliceHeader = SliceHeader::read(&mut descriptor_reader, true, sps_pps_provider);

        let residue = descriptor_reader.get_residue();
        let remaining_len: u64 = u64::from(len) - descriptor_reader.get_num_read_bytes();
        let mut remaining = vec![0u8; remaining_len.try_into().unwrap()];
        rdr.read_exact(&mut remaining).unwrap();

        Ok(IdrNalu {
            slice_header,
            residue,
            remaining,
            payload_size: len
        })
    }
}

impl Nalu for IdrNalu {
    fn get_payload_size(&self) -> u32 {
        self.payload_size
    }
    
    fn write(&self, wtr: &mut dyn Write, sps_pps_provider: &dyn SpsPpsProvider) {
        let mut descriptor_writer = DescriptorWriter::new(wtr);
        self.slice_header.write(&mut descriptor_writer, sps_pps_provider);
        
        descriptor_writer.append_u(self.residue.0, self.residue.1.into());
        descriptor_writer.append_all(&self.remaining);
        descriptor_writer.write_with_size_and_header(0x65);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl fmt::Display for IdrNalu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[IDR(slice_type={}, frame_num={}, pic_order_cnt_lsb={})]", self.slice_header.slice_type, self.slice_header.frame_num, self.slice_header.pic_order_cnt_lsb)
    }
}
