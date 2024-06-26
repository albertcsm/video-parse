use std::{any::Any, fmt, io::{self, Read, Write}};

use super::{descriptor_reader::DescriptorReader, descriptor_writer::DescriptorWriter, nalu::Nalu, sps_pps_provider::SpsPpsProvider, vui_parameters::VuiParameters};

#[derive(Debug, Clone)]
pub struct SpsNalu {
    pub profile_idc: u8,
    pub constraint_set0_flag: bool,
    pub constraint_set1_flag: bool,
    pub constraint_set2_flag: bool,
    pub constraint_set3_flag: bool,
    pub constraint_set4_flag: bool,
    pub constraint_set5_flag: bool,
    pub level_idc: u8,
    pub seq_parameter_set_id: u64,
    pub chroma_format_idc: u64,
    pub separate_colour_plane_flag: bool,
    pub bit_depth_luma_minus8: u64,
    pub bit_depth_chroma_minus8: u64,
    pub qpprime_y_zero_transform_bypass_flag: bool,
    pub seq_scaling_matrix_present_flag: bool,
    pub log2_max_frame_num_minus4: u64,
    pub pic_order_cnt_type: u64,
    pub log2_max_pic_order_cnt_lsb_minus4: u64,
    pub max_num_ref_frames: u64,
    pub gaps_in_frame_num_value_allowed_flag: bool,
    pub pic_width_in_mbs_minus1: u64,
    pub pic_height_in_map_units_minus1: u64,
    pub frame_mbs_only_flag: bool,
    pub mb_adaptive_frame_field_flag: bool,
    pub direct_8x8_inference_flag: bool,
    pub frame_cropping_flag: bool,
    pub frame_crop_left_offset: u64,
    pub frame_crop_right_offset: u64,
    pub frame_crop_top_offset: u64,
    pub frame_crop_bottom_offset: u64,
    pub vui_parameters: Option<VuiParameters>,
    pub payload_size: u32
}

impl SpsNalu {
    pub fn read(rdr: &mut impl Read, len: u32) -> io::Result<Self> {
        let mut descriptor_reader = DescriptorReader::new(rdr, len);
        let profile_idc = descriptor_reader.read_u8();
        let constraint_set0_flag = descriptor_reader.read_u1();
        let constraint_set1_flag = descriptor_reader.read_u1();
        let constraint_set2_flag = descriptor_reader.read_u1();
        let constraint_set3_flag = descriptor_reader.read_u1();
        let constraint_set4_flag = descriptor_reader.read_u1();
        let constraint_set5_flag = descriptor_reader.read_u1();
        let _reserved_zero_2bits = descriptor_reader.read_u(2);
        let level_idc: u8 = descriptor_reader.read_u8();
        let seq_parameter_set_id = descriptor_reader.read_ue_v();
        let mut chroma_format_idc = 0;
        let mut separate_colour_plane_flag = false;
        let mut bit_depth_luma_minus8 = 0;
        let mut bit_depth_chroma_minus8 = 0;
        let mut qpprime_y_zero_transform_bypass_flag = false;
        let mut seq_scaling_matrix_present_flag = false;
        match profile_idc {
            100 | 110 | 122 | 244 | 44 | 83 | 86 | 118 | 128 => {
                chroma_format_idc = descriptor_reader.read_ue_v();
                if chroma_format_idc == 3 {
                    separate_colour_plane_flag = descriptor_reader.read_u1();
                }
                bit_depth_luma_minus8 = descriptor_reader.read_ue_v();
                bit_depth_chroma_minus8 = descriptor_reader.read_ue_v();
                qpprime_y_zero_transform_bypass_flag = descriptor_reader.read_u1();
                seq_scaling_matrix_present_flag = descriptor_reader.read_u1();
                if seq_scaling_matrix_present_flag {
                    for _i in 0..=(if chroma_format_idc != 3 { 8 } else { 12 }) {
                        let seq_scaling_list_present_flag = descriptor_reader.read_u1();
                        if seq_scaling_list_present_flag {
                            todo!()
                        }
                    }
                }
            }
            _ => {}
        }
        let log2_max_frame_num_minus4 = descriptor_reader.read_ue_v();
        let pic_order_cnt_type = descriptor_reader.read_ue_v();
        let mut log2_max_pic_order_cnt_lsb_minus4 = 0;
        if pic_order_cnt_type == 0 {
            log2_max_pic_order_cnt_lsb_minus4 = descriptor_reader.read_ue_v();
        } else if pic_order_cnt_type == 1 {
            todo!()
        }
        let max_num_ref_frames = descriptor_reader.read_ue_v();
        let gaps_in_frame_num_value_allowed_flag = descriptor_reader.read_u1();
        let pic_width_in_mbs_minus1 = descriptor_reader.read_ue_v();
        let pic_height_in_map_units_minus1 = descriptor_reader.read_ue_v();
        let frame_mbs_only_flag = descriptor_reader.read_u1();
        let mut mb_adaptive_frame_field_flag = false;
        if !frame_mbs_only_flag {
            mb_adaptive_frame_field_flag = descriptor_reader.read_u1();
        }
        let direct_8x8_inference_flag = descriptor_reader.read_u1();
        let frame_cropping_flag = descriptor_reader.read_u1();
        let mut frame_crop_left_offset = 0;
        let mut frame_crop_right_offset = 0;
        let mut frame_crop_top_offset = 0;
        let mut frame_crop_bottom_offset = 0;
        if frame_cropping_flag {
            frame_crop_left_offset = descriptor_reader.read_ue_v();
            frame_crop_right_offset = descriptor_reader.read_ue_v();
            frame_crop_top_offset = descriptor_reader.read_ue_v();
            frame_crop_bottom_offset = descriptor_reader.read_ue_v();
        }
        let vui_parameters_present_flag = descriptor_reader.read_u1();
        let mut vui_parameters = Option::None;
        if vui_parameters_present_flag {
            vui_parameters = Option::Some(VuiParameters::read(&mut descriptor_reader));
        }

        Ok(SpsNalu {
            profile_idc,
            constraint_set0_flag,
            constraint_set1_flag,
            constraint_set2_flag,
            constraint_set3_flag,
            constraint_set4_flag,
            constraint_set5_flag,
            level_idc,
            seq_parameter_set_id,
            bit_depth_luma_minus8,
            bit_depth_chroma_minus8,
            qpprime_y_zero_transform_bypass_flag,
            seq_scaling_matrix_present_flag,
            chroma_format_idc,
            separate_colour_plane_flag,
            log2_max_frame_num_minus4,
            pic_order_cnt_type,
            log2_max_pic_order_cnt_lsb_minus4,
            max_num_ref_frames,
            gaps_in_frame_num_value_allowed_flag,
            pic_width_in_mbs_minus1,
            pic_height_in_map_units_minus1,
            frame_mbs_only_flag,
            mb_adaptive_frame_field_flag,
            direct_8x8_inference_flag,
            frame_cropping_flag,
            frame_crop_left_offset,
            frame_crop_right_offset,
            frame_crop_top_offset,
            frame_crop_bottom_offset,
            vui_parameters,
            payload_size: len
        })
    }
}

impl Nalu for SpsNalu {
    fn write(&self, wtr: &mut dyn Write, _sps_pps_provider: &dyn SpsPpsProvider) {
        let mut descriptor_writer = DescriptorWriter::new(wtr);
        descriptor_writer.append_u8(self.profile_idc);
        descriptor_writer.append_u1(self.constraint_set0_flag);
        descriptor_writer.append_u1(self.constraint_set1_flag);
        descriptor_writer.append_u1(self.constraint_set2_flag);
        descriptor_writer.append_u1(self.constraint_set3_flag);
        descriptor_writer.append_u1(self.constraint_set4_flag);
        descriptor_writer.append_u1(self.constraint_set5_flag);
        descriptor_writer.append_u(2, 0);
        descriptor_writer.append_u8(self.level_idc);
        descriptor_writer.append_ue_v(self.seq_parameter_set_id);
        match self.profile_idc {
            100 | 110 | 122 | 244 | 44 | 83 | 86 | 118 | 128 => {
                descriptor_writer.append_ue_v(self.chroma_format_idc);
                if self.chroma_format_idc == 3 {
                    descriptor_writer.append_u1(self.separate_colour_plane_flag);
                }
                descriptor_writer.append_ue_v(self.bit_depth_luma_minus8);
                descriptor_writer.append_ue_v(self.bit_depth_chroma_minus8);
                descriptor_writer.append_u1(self.qpprime_y_zero_transform_bypass_flag);
                descriptor_writer.append_u1(self.seq_scaling_matrix_present_flag);
                if self.seq_scaling_matrix_present_flag {
                    for _i in 0..=(if self.chroma_format_idc != 3 { 8 } else { 12 }) {
                        descriptor_writer.append_u1(false);
                    }
                }
            }
            _ => {}            
        }
        descriptor_writer.append_ue_v(self.log2_max_frame_num_minus4);
        descriptor_writer.append_ue_v(self.pic_order_cnt_type);
        if self.pic_order_cnt_type == 0 {
            descriptor_writer.append_ue_v(self.log2_max_pic_order_cnt_lsb_minus4);
        } else if self.pic_order_cnt_type == 1 {
            todo!()
        }
        descriptor_writer.append_ue_v(self.max_num_ref_frames);
        descriptor_writer.append_u1(self.gaps_in_frame_num_value_allowed_flag);
        descriptor_writer.append_ue_v(self.pic_width_in_mbs_minus1);
        descriptor_writer.append_ue_v(self.pic_height_in_map_units_minus1);  // 0_______ + uev=67=0000001000100
        descriptor_writer.append_u1(self.frame_mbs_only_flag);       // " 34 0010 001
        if !self.frame_mbs_only_flag {
            descriptor_writer.append_u1(self.mb_adaptive_frame_field_flag);
        }
        descriptor_writer.append_u1(self.direct_8x8_inference_flag); // # 35  0010 0011
        descriptor_writer.append_u1(self.frame_cropping_flag);
        if self.frame_cropping_flag {
            descriptor_writer.append_ue_v(self.frame_crop_left_offset);
            descriptor_writer.append_ue_v(self.frame_crop_right_offset);
            descriptor_writer.append_ue_v(self.frame_crop_top_offset);
            descriptor_writer.append_ue_v(self.frame_crop_bottom_offset);
        }
        match &self.vui_parameters {
            Some(vui_parameters) => {
                descriptor_writer.append_u1(true);
                vui_parameters.write(&mut descriptor_writer);
            },
            None => descriptor_writer.append_u1(false)
        }

        descriptor_writer.append_rbsp_trailing_bits();
        descriptor_writer.write_with_header(0x67);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl fmt::Display for SpsNalu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let vui_parameters = match &self.vui_parameters {
            Some(vui) => vui.to_string(),
            None => "n/a".to_owned(),
        };
        write!(f, "[SPS(profile_idc={}, level_idc={}, seq_parameter_set_id={}, pic_order_cnt_type={}, pic_width_in_mbs_minus1={}, pic_height_in_map_units_minus1={}, vui_parameters={})]", 
            self.profile_idc, self.level_idc, self.seq_parameter_set_id, self.pic_order_cnt_type, self.pic_width_in_mbs_minus1, self.pic_height_in_map_units_minus1, vui_parameters)
    }
}
