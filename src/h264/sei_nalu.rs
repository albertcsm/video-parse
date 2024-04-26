use std::{any::Any, fmt, io::{self, Read, Seek, Write}};

use super::{descriptor_reader::DescriptorReader, descriptor_writer::DescriptorWriter, nalu::Nalu};

pub struct SeiNalu {
    remaining: Vec<u8>,
    pub payload_size: u32
}

impl SeiNalu {
    pub fn read(rdr: &mut (impl Read + Seek), len: u32) -> io::Result<Self> {
        let descriptor_reader = DescriptorReader::new(rdr);
        let remaining_len: u64 = u64::from(len) - descriptor_reader.get_num_read_bytes();
        let mut remaining = vec![0u8; remaining_len.try_into().unwrap()];
        rdr.read_exact(&mut remaining).unwrap();

        Ok(SeiNalu {
            remaining,
            payload_size: len
        })
    }
}

impl Nalu for SeiNalu {
    fn get_payload_size(&self) -> u32 {
        self.payload_size
    }

    fn write(&self, wtr: &mut dyn Write) {
        let mut descriptor_writer = DescriptorWriter::new(wtr);
        descriptor_writer.append_all(&self.remaining);
        descriptor_writer.write_with_size_and_header(0x06);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl fmt::Display for SeiNalu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[SEI]")
    }
}
