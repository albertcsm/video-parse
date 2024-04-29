use std::{any::Any, fmt, io::{self, Read, Seek, Write}};

use super::{descriptor_reader::DescriptorReader, descriptor_writer::DescriptorWriter, nalu::Nalu, sps_pps_provider::SpsPpsProvider};

#[derive(Debug, Clone)]
pub struct PpsNalu {
    pub pic_parameter_set_id: u64,
    pub seq_parameter_set_id: u64,
    residue: (u8, u8),
    remaining: Vec<u8>,
    pub payload_size: u32
}

impl PpsNalu {
    pub fn read(rdr: &mut (impl Read + Seek), len: u32) -> io::Result<Self> {
        let mut descriptor_reader = DescriptorReader::new(rdr);
        let pic_parameter_set_id = descriptor_reader.read_ue_v();
        let seq_parameter_set_id = descriptor_reader.read_ue_v();

        let residue = descriptor_reader.get_residue();
        let remaining_len: u64 = u64::from(len) - descriptor_reader.get_num_read_bytes();
        let mut remaining = vec![0u8; remaining_len.try_into().unwrap()];
        rdr.read_exact(&mut remaining).unwrap();

        Ok(PpsNalu {
            pic_parameter_set_id,
            seq_parameter_set_id,
            residue,
            remaining,
            payload_size: len
        })
    }
}

impl Nalu for PpsNalu {
    fn get_payload_size(&self) -> u32 {
        self.payload_size
    }
    
    fn write(&self, wtr: &mut dyn Write, _sps_pps_provider: &dyn SpsPpsProvider) {
        let mut descriptor_writer = DescriptorWriter::new(wtr);
        descriptor_writer.append_ue_v(self.pic_parameter_set_id);
        descriptor_writer.append_ue_v(self.seq_parameter_set_id);

        descriptor_writer.append_u(self.residue.0, self.residue.1.into());
        descriptor_writer.append_all(&self.remaining);
        descriptor_writer.write_with_size_and_header(0x68);        
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl fmt::Display for PpsNalu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[PPS(pic_parameter_set_id={}, seq_parameter_set_id={})]", self.pic_parameter_set_id, self.seq_parameter_set_id)
    }
}
