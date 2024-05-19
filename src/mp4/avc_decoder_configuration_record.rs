use std::{fmt, io::{Cursor, Read, Write}, vec};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use crate::h264::{nalu::Nalu, pps_nalu::PpsNalu, sps_nalu::SpsNalu, sps_pps_provider::SpsPpsProvider};

pub struct AvcDecoderConfigurationRecord {
    pub configuration_version: u8,
    pub avc_profile_indication: u8,
    pub profile_compatibility: u8,
    pub avc_level_indication: u8,
    pub length_size_minus_one: u8,
    pub sequence_parameter_set_nal_units: Vec<SpsNalu>,
    pub picture_parameter_set_nal_units: Vec<PpsNalu>
}

impl AvcDecoderConfigurationRecord {
    pub fn read(rdr: &mut impl Read) -> (Self, u32) {
        let mut total_size: u32 = 7;
        let configuration_version = rdr.read_u8().unwrap();
        let avc_profile_indication = rdr.read_u8().unwrap();
        let profile_compatibility = rdr.read_u8().unwrap();
        let avc_level_indication = rdr.read_u8().unwrap();
        let length_size_minus_one = rdr.read_u8().unwrap() & 0b00000011;
        let num_of_sequence_parameter_sets = rdr.read_u8().unwrap() & 0b00011111;
        let mut sequence_parameter_set_nal_units = vec![];
        for _i in 0..num_of_sequence_parameter_sets {
            let sequence_parameter_set_length = rdr.read_u16::<BigEndian>().unwrap();
            let _nalu_header = rdr.read_u8().unwrap();
            let sps_unit = SpsNalu::read(rdr, (sequence_parameter_set_length - 1).into()).unwrap();
            sequence_parameter_set_nal_units.push(sps_unit);
            total_size += 2 + u32::from(sequence_parameter_set_length)
        }
        let num_of_picture_parameter_sets = rdr.read_u8().unwrap();
        let mut picture_parameter_set_nal_units = vec![];
        for _i in 0..num_of_picture_parameter_sets {
            let picture_parameter_set_length = rdr.read_u16::<BigEndian>().unwrap();
            let _nalu_header = rdr.read_u8().unwrap();
            let pps_unit = PpsNalu::read(rdr, (picture_parameter_set_length - 1).into()).unwrap();
            picture_parameter_set_nal_units.push(pps_unit);
            total_size += 2 + u32::from(picture_parameter_set_length)
        }
        match avc_profile_indication {
            100 | 110 | 122 | 144 => {
                todo!()
            }
            _ => {}
        }
        
        (AvcDecoderConfigurationRecord {
            configuration_version,
            avc_profile_indication,
            profile_compatibility,
            avc_level_indication,
            length_size_minus_one,
            sequence_parameter_set_nal_units,
            picture_parameter_set_nal_units
        }, total_size)
    }

    pub fn write(&self, wtr: &mut impl Write) {
        wtr.write_u8(self.configuration_version).unwrap();
        wtr.write_u8(self.avc_profile_indication).unwrap();
        wtr.write_u8(self.profile_compatibility).unwrap();
        wtr.write_u8(self.avc_level_indication).unwrap();
        wtr.write_u8(0b11111100 | self.length_size_minus_one).unwrap();
        wtr.write_u8(0b11100000 | u8::try_from(self.sequence_parameter_set_nal_units.len()).unwrap()).unwrap();
        for sequence_parameter_set_nal_unit in &self.sequence_parameter_set_nal_units {
            let bytes = sequence_parameter_set_nal_unit.to_bytes(self);
            wtr.write_u16::<BigEndian>(u16::try_from(bytes.len()).unwrap()).unwrap();
            wtr.write_all(&bytes).unwrap();
        }
        wtr.write_u8(self.picture_parameter_set_nal_units.len().try_into().unwrap()).unwrap();
        for picture_parameter_set_nal_unit in &self.picture_parameter_set_nal_units {
            let bytes = picture_parameter_set_nal_unit.to_bytes(self);
            wtr.write_u16::<BigEndian>(u16::try_from(bytes.len()).unwrap()).unwrap();
            wtr.write_all(&bytes).unwrap();
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut cursor = Cursor::new(Vec::new());
        self.write(&mut cursor);
        let buffer = cursor.into_inner();
        buffer
    }
}

impl SpsPpsProvider for AvcDecoderConfigurationRecord {
    fn get_pps(&self, id: u64) -> Option<&PpsNalu> {
        for nalu in &self.picture_parameter_set_nal_units {
            if nalu.pic_parameter_set_id == id {
                return Some(nalu)
            }
        }
        return None
    }

    fn get_sps(&self, id: u64) -> Option<&SpsNalu> {
        for nalu in &self.sequence_parameter_set_nal_units {
            if nalu.seq_parameter_set_id == id {
                return Some(nalu)
            }
        }
        return None
    }
}

impl fmt::Display for AvcDecoderConfigurationRecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sps = self.sequence_parameter_set_nal_units.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",");
        let pps = self.picture_parameter_set_nal_units.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",");
        write!(f, "sps: [{}], pps: [{}]", sps, pps)
    }
}
