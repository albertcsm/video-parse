use std::{any::Any, fmt, fs::File, io::{self, Read, Seek, Write}};

use byteorder::{BigEndian, WriteBytesExt};

use super::{atom::Atom, avc_decoder_configuration_record::AvcDecoderConfigurationRecord};

pub struct AvccBox {
    pub avc_decoder_configuration_record: AvcDecoderConfigurationRecord,
    pub remaining: Vec<u8>,
    pub payload_size: u64
}

impl AvccBox {
    pub fn read(rdr: &mut (impl Read + Seek), len: u64) -> io::Result<Self> {
        let (avc_decoder_configuration_record, read_size) = AvcDecoderConfigurationRecord::read(rdr);

        let mut remaining = vec![0u8; (len - u64::from(read_size)).try_into().unwrap()];
        rdr.read_exact(&mut remaining).unwrap();

        Ok(AvccBox {
            avc_decoder_configuration_record,
            remaining,
            payload_size: len
        })
    }
}

impl Atom for AvccBox {
    fn get_payload_size(&self) -> u64 {
        let bytes = self.avc_decoder_configuration_record.to_bytes();
        (bytes.len() + self.remaining.len()).try_into().unwrap()
    }

    fn write(&self, wtr: &mut File) {
        let bytes = self.avc_decoder_configuration_record.to_bytes();
        let total_size: usize = 8 + bytes.len() + self.remaining.len();
        wtr.write_u32::<BigEndian>(total_size.try_into().unwrap()).unwrap();
        wtr.write_all(b"avcC").unwrap();
        wtr.write_all(&bytes).unwrap();
        wtr.write_all(&self.remaining).unwrap();
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl fmt::Display for AvccBox {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "avcC({})", self.avc_decoder_configuration_record)
    }
}
