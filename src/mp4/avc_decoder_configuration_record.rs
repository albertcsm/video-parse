use std::{fmt, io::{Read, Write}, vec};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

pub struct AvcDecoderConfigurationRecord {
    pub configuration_version: u8,
    pub avc_profile_indication: u8,
    pub profile_compatibility: u8,
    pub avc_level_indication: u8,
    pub length_size_minus_one: u8,
    pub sequence_parameter_set_nal_units: Vec<Vec<u8>>,
    pub picture_parameter_set_nal_units: Vec<Vec<u8>>
}

impl AvcDecoderConfigurationRecord {
    pub fn read(rdr: &mut impl Read) -> Self {
        let configuration_version = rdr.read_u8().unwrap();
        let avc_profile_indication = rdr.read_u8().unwrap();
        let profile_compatibility = rdr.read_u8().unwrap();
        let avc_level_indication = rdr.read_u8().unwrap();
        let length_size_minus_one = rdr.read_u8().unwrap() & 0b00000011;
        let num_of_sequence_parameter_sets = rdr.read_u8().unwrap() & 0b00011111;
        let mut sequence_parameter_set_nal_units: Vec<Vec<u8>> = vec![];
        for _i in 0..num_of_sequence_parameter_sets {
            let sequence_parameter_set_length = rdr.read_u16::<BigEndian>().unwrap();
            let mut sequence_parameter_set_nal_unit: Vec<u8> = vec![0; sequence_parameter_set_length.into()];
            rdr.read_exact(&mut sequence_parameter_set_nal_unit).unwrap();
            sequence_parameter_set_nal_units.push(sequence_parameter_set_nal_unit);
        }
        let num_of_picture_parameter_sets = rdr.read_u8().unwrap();
        let mut picture_parameter_set_nal_units: Vec<Vec<u8>> = vec![];
        for _i in 0..num_of_picture_parameter_sets {
            let picture_parameter_set_length = rdr.read_u16::<BigEndian>().unwrap();
            let mut picture_parameter_set_nal_unit: Vec<u8> = vec![0; picture_parameter_set_length.into()];
            rdr.read_exact(&mut picture_parameter_set_nal_unit).unwrap();
            picture_parameter_set_nal_units.push(picture_parameter_set_nal_unit);
        }
        match avc_profile_indication {
            100 | 110 | 122 | 144 => {
                todo!()
            }
            _ => {}
        }
        
        AvcDecoderConfigurationRecord {
            configuration_version,
            avc_profile_indication,
            profile_compatibility,
            avc_level_indication,
            length_size_minus_one,
            sequence_parameter_set_nal_units,
            picture_parameter_set_nal_units
        }
    }

    pub fn size(&self) -> u64 {
        let mut total = 7;
        for i in &self.sequence_parameter_set_nal_units {
            total += 2 + i.len();
        }
        for i in &self.picture_parameter_set_nal_units {
            total += 2 + i.len();
        }
        total.try_into().unwrap()
    }

    pub fn write(&self, wtr: &mut impl Write) {
        wtr.write_u8(self.configuration_version).unwrap();
        wtr.write_u8(self.avc_profile_indication).unwrap();
        wtr.write_u8(self.profile_compatibility).unwrap();
        wtr.write_u8(self.avc_level_indication).unwrap();
        wtr.write_u8(0b11111100 | self.length_size_minus_one).unwrap();
        wtr.write_u8(0b11100000 | u8::try_from(self.sequence_parameter_set_nal_units.len()).unwrap()).unwrap();
        for sequence_parameter_set_nal_unit in &self.sequence_parameter_set_nal_units {
            wtr.write_u16::<BigEndian>(sequence_parameter_set_nal_unit.len().try_into().unwrap()).unwrap();
            wtr.write_all(sequence_parameter_set_nal_unit).unwrap();
        }
        wtr.write_u8(self.picture_parameter_set_nal_units.len().try_into().unwrap()).unwrap();
        for picture_parameter_set_nal_unit in &self.picture_parameter_set_nal_units {
            wtr.write_u16::<BigEndian>(picture_parameter_set_nal_unit.len().try_into().unwrap()).unwrap();
            wtr.write_all(picture_parameter_set_nal_unit).unwrap();
        }
    }
}

impl fmt::Display for AvcDecoderConfigurationRecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "sps: {}, pps: {}", self.sequence_parameter_set_nal_units.len(), self.picture_parameter_set_nal_units.len())
    }
}
