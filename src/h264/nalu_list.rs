use std::fs::File;

use byteorder::{BigEndian, ReadBytesExt};

use super::{delim_nalu::DelimNalu, idr_nalu::IdrNalu, nalu::Nalu, non_idr_nalu::NonIdrNalu, pps_nalu::PpsNalu, sei_nalu::SeiNalu, sps_nalu::SpsNalu, sps_pps_provider::SpsPpsProvider, unknown_nalu::UnknownNalu};

pub struct NaluList {
    units: Vec<Box<dyn Nalu>>
}

impl NaluList {
    pub fn read(rdr: &mut File, len: u64) -> Self {
        let mut list = NaluList {
            units: vec![]
        };
        let mut read_len: u64 = 0;
        loop {
            let unit_payload_size = list.read_nalu(rdr);
            read_len += 5 + u64::from(unit_payload_size);
            if len != 0 && read_len >= len {
                break;
            }
        }
        list
    }

    pub fn get_units(&self) -> &Vec<Box<dyn Nalu>> {
        &self.units
    }

    fn read_nalu(&mut self, rdr: &mut File) -> u32 {
        let size = rdr.read_u32::<BigEndian>().unwrap();
        let header = rdr.read_u8().unwrap();
        let _nal_ref_idc = (header & 0b01100000) >> 5;
        let nal_unit_type = header & 0b00011111;
        let payload_size = size - 1;
        match nal_unit_type {
            1 => {
                let unit = NonIdrNalu::read(rdr, payload_size, self).unwrap();
                let payload_size = unit.payload_size;
                self.units.push(Box::new(unit));
                payload_size
            },
            5 => {
                let unit = IdrNalu::read(rdr, payload_size, self).unwrap();
                let payload_size = unit.payload_size;
                self.units.push(Box::new(unit));
                payload_size
            },
            6 => {
                let unit = SeiNalu::read(rdr, payload_size).unwrap();
                let payload_size = unit.payload_size;
                self.units.push(Box::new(unit));
                payload_size
            },
            7 => {
                let unit = SpsNalu::read(rdr, payload_size).unwrap();
                let payload_size = unit.payload_size;
                self.units.push(Box::new(unit));
                payload_size
            },
            8 => {
                let unit = PpsNalu::read(rdr, payload_size).unwrap();
                let payload_size = unit.payload_size;
                self.units.push(Box::new(unit));
                payload_size
            },
            9 => {
                let unit = DelimNalu::read(rdr, payload_size).unwrap();
                let payload_size = unit.payload_size;
                self.units.push(Box::new(unit));
                payload_size
            },
            _ => {
                let unit = UnknownNalu::read(rdr, payload_size, nal_unit_type).unwrap();
                let payload_size = unit.payload_size;
                self.units.push(Box::new(unit));
                payload_size
            }
        }
    }
}

impl SpsPpsProvider for NaluList {
    fn get_sps(&self, id: u64) -> Option<&SpsNalu> {
        for unit in &self.units {
            let any_unit = unit.as_any();
            match any_unit.downcast_ref::<SpsNalu>() {
                Some(sps_unit) => {
                    if sps_unit.seq_parameter_set_id == id {
                        return Some(sps_unit)
                    }
                },
                _ => {}
            }
        }
        None
    }

    fn get_pps(&self, id: u64) -> Option<&PpsNalu> {
        for unit in &self.units {
            let any_unit = unit.as_any();
            match any_unit.downcast_ref::<PpsNalu>() {
                Some(pps_unit) => {
                    if pps_unit.pic_parameter_set_id == id {
                        return Some(pps_unit)
                    }
                },
                _ => {}
            }
        }
        None
    }
}