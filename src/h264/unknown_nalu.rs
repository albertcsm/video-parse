use std::{any::Any, fmt, io::{self, Read, Seek, Write}};

use super::{descriptor_reader::DescriptorReader, descriptor_writer::DescriptorWriter, nalu::Nalu, opaque_data::OpaqueData, sps_pps_provider::SpsPpsProvider};

pub struct UnknownNalu {
    pub nal_unit_type: u8,
    remaining: OpaqueData,
    pub payload_size: u32
}

impl UnknownNalu {
    pub fn read(rdr: &mut (impl Read + Seek), len: u32, nal_unit_type: u8) -> io::Result<Self> {
        let mut descriptor_reader = DescriptorReader::new(rdr, len);
        let remaining = descriptor_reader.read_to_end();
        descriptor_reader.read_rbsp_trailing_bits();

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
        descriptor_writer.append_rbsp_trailing_bits();
        descriptor_writer.write_with_header(self.nal_unit_type);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl fmt::Debug for UnknownNalu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UnknownNalu")
            .field("nal_unit_type", &self.nal_unit_type)
            .finish()
    }
}
