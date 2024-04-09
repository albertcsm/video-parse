use std::io::{Read, Seek};

use byteorder::{BigEndian, ReadBytesExt};

use super::{delim_nalu::DelimNalu, idr_nalu::IdrNalu, nalu::Nalu, non_idr_nalu::NonIdrNalu, pps_nalu::PpsNalu, sei_nalu::SeiNalu, sps_nalu::SpsNalu, unknown_nalu};

pub fn read_nalu(rdr: &mut (impl Read + Seek)) -> Option<Box<dyn Nalu>> {
    let size = rdr.read_u32::<BigEndian>().unwrap();
    let header = rdr.read_u8().unwrap();
    let _nal_ref_idc = (header & 0b01100000) >> 5;
    let nal_unit_type = header & 0b00011111;
    let payload_size = size - 1;
    match nal_unit_type {
        1 => Some(Box::new(NonIdrNalu::read(rdr, payload_size).unwrap())),
        5 => Some(Box::new(IdrNalu::read(rdr, payload_size).unwrap())),
        6 => Some(Box::new(SeiNalu::read(rdr, payload_size).unwrap())),
        7 => Some(Box::new(SpsNalu::read(rdr, payload_size).unwrap())),
        8 => Some(Box::new(PpsNalu::read(rdr, payload_size).unwrap())),
        9 => Some(Box::new(DelimNalu::read(rdr, payload_size).unwrap())),
        _ => Some(Box::new(unknown_nalu::UnknownNalu::read(rdr, payload_size, nal_unit_type).unwrap()))
    }
}

pub fn read_nalus(rdr: &mut (impl Read + Seek), len: u64) -> Vec<Box<dyn Nalu>> {
    let mut vec = Vec::new();
    let mut read_len: u64 = 0;
    while let Some(atom) = read_nalu(rdr) {
        read_len += 5 + u64::from(atom.get_payload_size());
        vec.push(atom);
        if len != 0 && read_len >= len {
            break;
        }
    }
    vec
}
