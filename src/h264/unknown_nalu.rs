use std::{any::Any, fmt, io::{self, Read, Seek, Write}};

use super::{descriptor_reader::DescriptorReader, descriptor_writer::DescriptorWriter, nalu::Nalu, sps_pps_provider::SpsPpsProvider};

pub struct UnknownNalu {
    pub nal_unit_type: u8,
    remaining: Vec<u8>,
    pub payload_size: u32
}

impl UnknownNalu {
    pub fn read(rdr: &mut (impl Read + Seek), len: u32, nal_unit_type: u8) -> io::Result<Self> {
        let descriptor_reader = DescriptorReader::new(rdr);
        let remaining_len: u64 = u64::from(len) - descriptor_reader.get_num_read_bytes();
        let mut remaining = vec![0u8; remaining_len.try_into().unwrap()];
        rdr.read_exact(&mut remaining).unwrap();

        Ok(UnknownNalu {
            nal_unit_type,
            remaining,
            payload_size: len
        })
    }
}

impl Nalu for UnknownNalu {
    fn write(&self, wtr: &mut dyn Write, _sps_pps_provider: &dyn SpsPpsProvider) {
        let mut descriptor_writer = DescriptorWriter::new(wtr);
        descriptor_writer.append_all(&self.remaining);
        descriptor_writer.write_with_header(self.nal_unit_type);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl fmt::Display for UnknownNalu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[NALU {}]", self.nal_unit_type)
    }
}
