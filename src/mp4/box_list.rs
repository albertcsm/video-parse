use std::fs::File;

use byteorder::{BigEndian, ReadBytesExt};

use super::{atom::Atom, four_cc::FourCC, ftyp_box, mdat_box, moov_box, mvhd_box, unknown_box};

pub struct BoxList {
    pub boxes: Vec<Box<dyn Atom>>
}

impl BoxList {
    pub fn read(rdr: &mut File, len: u64) -> Self {
        let mut boxes = Vec::new();
        let mut read_len = 0;
        while let Some(atom) = BoxList::read_atom(rdr) {
            read_len += 8 + &atom.get_payload_size();      // TODO: handle large header
            boxes.push(atom);
            if len != 0 && read_len >= len {
                break;
            }
        }
        BoxList {
            boxes
        }
    }

    pub fn write(&self, wtr: &mut File) {
        for atom in &self.boxes {
            atom.write(wtr);
        }
    }

    fn read_atom(rdr: &mut File) -> Option<Box<dyn Atom>> {
        let size_u32 = rdr.read_u32::<BigEndian>();
        let size: u64 = match size_u32 {
            Ok(1) => rdr.read_u64::<BigEndian>().unwrap(),
            Ok(s) => s.into(),
            Err(_) => return None,
        };

        let payload_size = size - 8;
        let boxtype = FourCC::read(rdr).unwrap();
        let name = boxtype.to_string();
        let name_str = name.as_str();

        match name_str {
            "ftyp" => Some(Box::new(ftyp_box::FtypBox::read(rdr, payload_size).unwrap())),
            "mdat" => Some(Box::new(mdat_box::MdatBox::read(rdr, payload_size).unwrap())),
            "moov" => Some(Box::new(moov_box::MoovBox::read(rdr, payload_size).unwrap())),
            "mvhd" => Some(Box::new(mvhd_box::MvhdBox::read(rdr, payload_size).unwrap())),
            _ => Some(Box::new(unknown_box::UnknownBox::read(rdr, payload_size, boxtype).unwrap()))
        }
    }
}
