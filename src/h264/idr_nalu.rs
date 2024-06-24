use std::{any::Any, fmt, io::{self, Read, Seek, Write}};

use super::{descriptor_reader::DescriptorReader, descriptor_writer::DescriptorWriter, nalu::Nalu, opaque_data::OpaqueData, slice_header::SliceHeader, sps_pps_provider::SpsPpsProvider};

pub struct IdrNalu {
    pub slice_header: SliceHeader,
    remaining: OpaqueData,
    pub payload_size: u32
}

impl IdrNalu {
    pub fn read<'a>(rdr: &mut (impl Read + Seek), len: u32, sps_pps_provider: &impl SpsPpsProvider) -> io::Result<Self> {
        let mut descriptor_reader = DescriptorReader::new(rdr, len);
        let slice_header: SliceHeader = SliceHeader::read(&mut descriptor_reader, true, sps_pps_provider);
        let remaining = descriptor_reader.read_to_end();
        descriptor_reader.read_rbsp_trailing_bits();

        Ok(IdrNalu {
            slice_header,
            remaining,
            payload_size: len
        })
    }
}

impl Nalu for IdrNalu {
    fn write(&self, wtr: &mut dyn Write, sps_pps_provider: &dyn SpsPpsProvider) {
        let mut descriptor_writer = DescriptorWriter::new(wtr);
        self.slice_header.write(&mut descriptor_writer, sps_pps_provider);
        descriptor_writer.append_all(&self.remaining);
        descriptor_writer.append_rbsp_trailing_bits();
        descriptor_writer.write_with_header(0x65);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl fmt::Debug for IdrNalu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("IdrNalu")
            .field("slice_header", &self.slice_header)
            .finish()
    }
}
