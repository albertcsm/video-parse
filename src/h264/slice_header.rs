use super::{descriptor_reader::DescriptorReader, descriptor_writer::DescriptorWriter, sps_pps_provider::SpsPpsProvider};

pub struct SliceHeader {
    pub idr_pic_flag: bool,
    pub first_mb_in_slice: u64,
    pub slice_type: u64,
    pub pic_parameter_set_id: u64,
    pub colour_plane_id: u8,
    pub frame_num: u64,
    pub field_pic_flag: bool,
    pub bottom_field_flag: bool,
    pub idr_pic_id: u64,
    pub pic_order_cnt_lsb: u64,
}

impl SliceHeader {
    pub fn read(descriptor_reader: &mut DescriptorReader, idr_pic_flag: bool, sps_pps_provider: &impl SpsPpsProvider) -> Self {
        let first_mb_in_slice = descriptor_reader.read_ue_v();
        let slice_type = descriptor_reader.read_ue_v();
        let pic_parameter_set_id = descriptor_reader.read_ue_v();
        let pps = sps_pps_provider.get_pps(pic_parameter_set_id).unwrap();
        let sps = sps_pps_provider.get_sps(pps.seq_parameter_set_id).unwrap();
        let mut colour_plane_id: u8 = 0;
        if sps.separate_colour_plane_flag {
            colour_plane_id = descriptor_reader.read_u(2).try_into().unwrap();
        }
        let frame_num_bits = sps.log2_max_frame_num_minus4 + 4;
        let frame_num = descriptor_reader.read_u(u8::try_from(frame_num_bits).unwrap());
        let mut field_pic_flag = false;
        let mut bottom_field_flag = false;
        if !sps.frame_mbs_only_flag {
            field_pic_flag = descriptor_reader.read_u1();
            if field_pic_flag {
                bottom_field_flag = descriptor_reader.read_u1();
            }
        }
        let mut idr_pic_id = 0;
        if idr_pic_flag {
            idr_pic_id = descriptor_reader.read_ue_v();
        }
        let mut pic_order_cnt_lsb = 0;
        if sps.pic_order_cnt_type == 0 {
            let pic_order_cnt_lsb_bits = sps.log2_max_pic_order_cnt_lsb_minus4 + 4;
            pic_order_cnt_lsb = descriptor_reader.read_u(u8::try_from(pic_order_cnt_lsb_bits).unwrap());
        }
        SliceHeader {
            idr_pic_flag,
            first_mb_in_slice,
            slice_type,
            pic_parameter_set_id,
            colour_plane_id,
            frame_num,
            field_pic_flag,
            bottom_field_flag,
            idr_pic_id,
            pic_order_cnt_lsb
        }
    }

    pub fn write(&self, descriptor_writer: &mut DescriptorWriter, sps_pps_provider: &dyn SpsPpsProvider) {
        let pps = sps_pps_provider.get_pps(self.pic_parameter_set_id).unwrap();
        let sps = sps_pps_provider.get_sps(pps.seq_parameter_set_id).unwrap();

        descriptor_writer.append_ue_v(self.first_mb_in_slice);
        descriptor_writer.append_ue_v(self.slice_type);
        descriptor_writer.append_ue_v(self.pic_parameter_set_id);
        if sps.separate_colour_plane_flag {
            descriptor_writer.append_u(2, self.colour_plane_id.into());
        }
        let frame_num_bits = sps.log2_max_frame_num_minus4 + 4;
        descriptor_writer.append_u(u8::try_from(frame_num_bits).unwrap(), self.frame_num);
        if !sps.frame_mbs_only_flag {
            descriptor_writer.append_u1(self.field_pic_flag);
            if self.field_pic_flag {
                descriptor_writer.append_u1(self.bottom_field_flag);
            }
        }
        if self.idr_pic_flag {
            descriptor_writer.append_ue_v(self.idr_pic_id);
        }
        if sps.pic_order_cnt_type == 0 {
            let pic_order_cnt_lsb_bits = sps.log2_max_pic_order_cnt_lsb_minus4 + 4;
            descriptor_writer.append_u(u8::try_from(pic_order_cnt_lsb_bits).unwrap(), self.pic_order_cnt_lsb);
        }
    }
}