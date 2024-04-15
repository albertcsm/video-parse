
use super::{descriptor_reader::DescriptorReader, sps_pps_provider::SpsPpsProvider};

pub struct SliceHeader {
    pub first_mb_in_slice: u64,
    pub slice_type: u64,
    pub pic_parameter_set_id: u64,
    pub frame_num: u64,
    pub pic_order_cnt_lsb: u64,
}

impl SliceHeader {
    pub fn read(descriptor_reader: &mut DescriptorReader, idr_pic_flag: bool, sps_pps_provider: &impl SpsPpsProvider) -> Self {
        let first_mb_in_slice = descriptor_reader.read_ue_v();
        let slice_type = descriptor_reader.read_ue_v();
        let pic_parameter_set_id = descriptor_reader.read_ue_v();
        let pps = sps_pps_provider.get_pps(pic_parameter_set_id).unwrap();
        let sps = sps_pps_provider.get_sps(pps.seq_parameter_set_id).unwrap();
        if sps.separate_colour_plane_flag {
            let _colour_plane_id = descriptor_reader.read_u(2);
        }
        let frame_num_bits = sps.log2_max_frame_num_minus4 + 4;
        let frame_num = descriptor_reader.read_u(u8::try_from(frame_num_bits).unwrap());
        if !sps.frame_mbs_only_flag {
            let field_pic_flag = descriptor_reader.read_u1();
            if field_pic_flag {
                let _bottom_field_flag = descriptor_reader.read_u1();
            }
        }
        if idr_pic_flag {
            let _idr_pic_id = descriptor_reader.read_ue_v();
        }
        let mut pic_order_cnt_lsb = 0;
        if sps.pic_order_cnt_type == 0 {
            let pic_order_cnt_lsb_bits = sps.log2_max_pic_order_cnt_lsb_minus4 + 4;
            pic_order_cnt_lsb = descriptor_reader.read_u(u8::try_from(pic_order_cnt_lsb_bits).unwrap());
        }
        SliceHeader {
            first_mb_in_slice,
            slice_type,
            pic_parameter_set_id,
            frame_num,
            pic_order_cnt_lsb,
        }
    }
}