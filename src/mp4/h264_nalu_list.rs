use std::{fmt, fs::File, io::Write};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use crate::h264::{delim_nalu::DelimNalu, idr_nalu::IdrNalu, nalu::Nalu, non_idr_nalu::NonIdrNalu, pps_nalu::PpsNalu, sei_nalu::SeiNalu, sps_nalu::SpsNalu, sps_pps_provider::SpsPpsProvider, unknown_nalu::UnknownNalu};

pub struct H264NaluList {
    pub units: Vec<Box<dyn Nalu>>
}

impl H264NaluList {
    pub fn read(rdr: &mut File, len: u64) -> Self {
        let mut list = H264NaluList {
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

    pub fn write(&self, wtr: &mut dyn Write) -> Vec<u32>{
        let mut sample_offsets: Vec<u32> = vec![];
        let mut offset: u32 = 0;
        let mut size: u32 = 0;
        for unit in &self.units {
            // remove SEI, SPS, PPS units
            // if let Some(_) = unit.as_any().downcast_ref::<SeiNalu>() {
            //     continue;
            // } else if let Some(_) = unit.as_any().downcast_ref::<SpsNalu>() {
            //     continue;
            // } else if let Some(_) = unit.as_any().downcast_ref::<PpsNalu>() {
            //     continue;
            // }

            let bytes = unit.to_bytes(self);
            wtr.write_u32::<BigEndian>(u32::try_from(bytes.len()).unwrap()).unwrap();   
            wtr.write_all(&bytes).unwrap();

            if let Some(_) = unit.as_any().downcast_ref::<DelimNalu>() {
                size += 4 + u32::try_from(bytes.len()).unwrap();    // not added to offset until next sample
            } else if let Some(_) = unit.as_any().downcast_ref::<IdrNalu>() {
                sample_offsets.push(offset);
                offset = size + 4 + u32::try_from(bytes.len()).unwrap();  // offset for next sample, count earlier delim nalu
                size = 0;
            } else if let Some(_) = unit.as_any().downcast_ref::<NonIdrNalu>() {
                sample_offsets.push(offset);
                offset = size + 4 + u32::try_from(bytes.len()).unwrap();  // offset for next sample, count earlier delim nalu
                size = 0;
            } else {
                offset += 4 + u32::try_from(bytes.len()).unwrap();
            }
        }
        sample_offsets
    }

    fn read_nalu(&mut self, rdr: &mut File) -> u32 {
        let size = rdr.read_u32::<BigEndian>().unwrap();
        let header = rdr.read_u8().unwrap();
        let _nal_ref_idc = (header & 0b01100000) >> 5;
        let nal_unit_type = header & 0b00011111;
        let payload_size = size - 1;
        match nal_unit_type {
            1 => {
                let unit = NonIdrNalu::read(rdr, payload_size, header, self).unwrap();
                self.units.push(Box::new(unit));
            },
            5 => {
                let unit = IdrNalu::read(rdr, payload_size, self).unwrap();
                self.units.push(Box::new(unit));
            },
            6 => {
                let unit = SeiNalu::read(rdr, payload_size).unwrap();
                self.units.push(Box::new(unit));
            },
            7 => {
                let unit = SpsNalu::read(rdr, payload_size).unwrap();
                self.units.push(Box::new(unit));
            },
            8 => {
                let unit = PpsNalu::read(rdr, payload_size).unwrap();
                self.units.push(Box::new(unit));
            },
            9 => {
                let unit = DelimNalu::read(rdr, payload_size).unwrap();
                self.units.push(Box::new(unit));
            },
            _ => {
                let unit = UnknownNalu::read(rdr, payload_size, nal_unit_type).unwrap();
                self.units.push(Box::new(unit));
            }
        }
        payload_size
    }
}

impl SpsPpsProvider for H264NaluList {
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

impl fmt::Debug for H264NaluList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list()
            .entries(&self.units)
            .finish()
    }
}
