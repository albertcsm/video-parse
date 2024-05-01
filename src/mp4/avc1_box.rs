use std::{any::Any, fmt, fs::File, io::{self, Read, Write}};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use super::{atom::Atom, box_list::BoxList};

pub struct Avc1Box {
    pub data_reference_index: u16,
    pub visual_sample_entry_reserved: u16,
    pub width: u16,
    pub height: u16,
    pub compressorname: [u8; 32],
    pub box_list: BoxList,
    pub payload_size: u64
}

impl Avc1Box {
    pub fn read(rdr: &mut File, len: u64) -> io::Result<Self> {
        let mut _reserved: [u8; 6] = [0; 6];
        rdr.read(&mut _reserved).unwrap();

        let data_reference_index = rdr.read_u16::<BigEndian>().unwrap();

        let _pre_defined = rdr.read_u16::<BigEndian>();
        let visual_sample_entry_reserved = rdr.read_u16::<BigEndian>().unwrap();
        for _i in 0..3 {
            rdr.read_u32::<BigEndian>()?;
        }
        
        let width = rdr.read_u16::<BigEndian>()?;
        let height = rdr.read_u16::<BigEndian>()?;

        let _horizresolution = rdr.read_u32::<BigEndian>()?;
        let _vertresolution = rdr.read_u32::<BigEndian>()?;

        let _reserved2 = rdr.read_u32::<BigEndian>()?;

        let _frame_count = rdr.read_u16::<BigEndian>();

        let mut compressorname: [u8; 32] = [0; 32];
        rdr.read(&mut compressorname).unwrap();

        let _depth = rdr.read_u16::<BigEndian>();

        let _pre_defined2 = rdr.read_i16::<BigEndian>();

        let box_list = BoxList::read(rdr, len - 78);
        Ok(Avc1Box {
            data_reference_index,
            visual_sample_entry_reserved,
            width,
            height,
            compressorname,
            box_list,
            payload_size: len
        })
    }
}

impl Atom for Avc1Box {
    fn get_payload_size(&self) -> u64 {
        self.payload_size
    }

    fn write(&self, wtr: &mut File) {
        let total_size = 8 + self.payload_size;
        wtr.write_u32::<BigEndian>(total_size.try_into().unwrap()).unwrap();
        wtr.write_all(b"avc1").unwrap();

        let reserved: [u8; 6] = [0; 6];
        wtr.write_all(&reserved).unwrap();
        wtr.write_u16::<BigEndian>(self.data_reference_index).unwrap();
        wtr.write_u16::<BigEndian>(0).unwrap();             // pre_defined
        wtr.write_u16::<BigEndian>(self.visual_sample_entry_reserved).unwrap();

        for _i in 0..3 {
            wtr.write_u32::<BigEndian>(0).unwrap();
        }

        wtr.write_u16::<BigEndian>(self.width).unwrap();
        wtr.write_u16::<BigEndian>(self.height).unwrap();
        wtr.write_u32::<BigEndian>(0x00480000).unwrap();    // horizresolution
        wtr.write_u32::<BigEndian>(0x00480000).unwrap();    // vertresolution
        wtr.write_u32::<BigEndian>(0).unwrap();             // reserved
        wtr.write_u16::<BigEndian>(1).unwrap();             // frame_count
        wtr.write_all(&self.compressorname).unwrap();
        wtr.write_u16::<BigEndian>(0x0018).unwrap();        // depth
        wtr.write_i16::<BigEndian>(-1).unwrap();            // pre_defined

        self.box_list.write(wtr);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl fmt::Display for Avc1Box {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let children = self.box_list.boxes.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",");
        write!(f, "avc1(width={}, height={}, {})", self.width, self.height, children)
    }
}
