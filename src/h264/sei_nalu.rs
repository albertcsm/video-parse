use std::{any::Any, fmt, io::{self, Read, Seek, Write}};

use super::{descriptor_reader::DescriptorReader, descriptor_writer::DescriptorWriter, nalu::Nalu, opaque_data::OpaqueData, sps_pps_provider::SpsPpsProvider};

pub struct SeiNalu {
    remaining: OpaqueData,
    pub payload_size: u32
}

impl SeiNalu {
    pub fn read(rdr: &mut (impl Read + Seek), len: u32) -> io::Result<Self> {
        let mut descriptor_reader = DescriptorReader::new(rdr, len);
        let remaining = descriptor_reader.read_to_end();
        descriptor_reader.read_rbsp_trailing_bits();

        Ok(SeiNalu {
            remaining,
            payload_size: len
        })
    }
}

impl Nalu for SeiNalu {
    fn write(&self, wtr: &mut dyn Write, _sps_pps_provider: &dyn SpsPpsProvider) {
        let mut descriptor_writer = DescriptorWriter::new(wtr);
        descriptor_writer.append_all(&self.remaining);
        descriptor_writer.append_rbsp_trailing_bits();
        descriptor_writer.write_with_header(0x06);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl fmt::Debug for SeiNalu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SeiNalu")
            .finish()
    }
}
