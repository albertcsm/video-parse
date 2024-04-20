use std::{any::Any, fmt, fs::File, io::{self, Read, Seek}};


use super::{descriptor_reader::DescriptorReader, descriptor_writer::DescriptorWriter, nalu::Nalu};

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
    residue: (u8, u8),
    remaining: Vec<u8>,
    pub payload_size: u32
}

pub struct VuiParameters {
    pub aspect_ratio_info_present_flag: bool,
    pub aspect_ratio_idc: u8,
    pub sar_width: u16,
    pub sar_height: u16,
    pub overscan_info_present_flag: bool,
    pub overscan_appropriate_flag: bool,
    pub video_signal_type_present_flag: bool,
    pub video_format: u64,
    pub video_full_range_flag: bool,
    pub colour_description_present_flag: bool,
    pub colour_primaries: u8,
    pub transfer_characteristics: u8,
    pub matrix_coefficients: u8,
    pub chroma_loc_info_present_flag: bool,
    pub timing_info_present_flag: bool,
    pub num_units_in_tick: Option<u32>,
    pub time_scale: Option<u32>,
    pub fixed_frame_rate_flag: Option<bool>
}

impl SpsNalu {
    pub fn read(rdr: &mut (impl Read + Seek), len: u32) -> io::Result<Self> {
        let mut descriptor_reader = DescriptorReader::new(rdr);
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
            vui_parameters = Option::Some(SpsNalu::read_vui(&mut descriptor_reader));
        }

        let residue = descriptor_reader.get_residue();
        let remaining_len: u64 = u64::from(len) - descriptor_reader.get_num_read_bytes();
        let mut remaining = vec![0u8; remaining_len.try_into().unwrap()];
        rdr.read_exact(&mut remaining).unwrap();

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
            residue,
            remaining,
            payload_size: len
        })
    }

    fn read_vui(descriptor_reader: &mut DescriptorReader) -> VuiParameters {
        let aspect_ratio_info_present_flag = descriptor_reader.read_u1();
        let mut aspect_ratio_idc = 0;
        let mut sar_width = 0;
        let mut sar_height = 0;
        if aspect_ratio_info_present_flag {
            aspect_ratio_idc = descriptor_reader.read_u8();
            if aspect_ratio_idc == 255 {    // Extended_SAR
                sar_width = descriptor_reader.read_u16();
                sar_height = descriptor_reader.read_u16();
            }
        }
        let overscan_info_present_flag = descriptor_reader.read_u1();
        let mut overscan_appropriate_flag = false;
        if overscan_info_present_flag {
            overscan_appropriate_flag = descriptor_reader.read_u1();
        }
        let video_signal_type_present_flag = descriptor_reader.read_u1();
        let mut video_format = 0;
        let mut video_full_range_flag = false;
        let mut colour_description_present_flag = false;
        let mut colour_primaries = 0;
        let mut transfer_characteristics = 0;
        let mut matrix_coefficients = 0;
        if video_signal_type_present_flag {
            video_format = descriptor_reader.read_u(3);
            video_full_range_flag = descriptor_reader.read_u1();
            colour_description_present_flag = descriptor_reader.read_u1();
            if colour_description_present_flag {
                colour_primaries = descriptor_reader.read_u8();
                transfer_characteristics = descriptor_reader.read_u8();
                matrix_coefficients = descriptor_reader.read_u8();
            }
        }
        let chroma_loc_info_present_flag = descriptor_reader.read_u1();
        if chroma_loc_info_present_flag {
            todo!()
        }
        let timing_info_present_flag = descriptor_reader.read_u1();
        let mut num_units_in_tick = Option::None;
        let mut time_scale = Option::None;
        let mut fixed_frame_rate_flag = Option::None;
        if timing_info_present_flag {
            num_units_in_tick = Option::Some(descriptor_reader.read_u32());
            time_scale = Option::Some(descriptor_reader.read_u32());
            fixed_frame_rate_flag = Option::Some(descriptor_reader.read_u1());
        }
        VuiParameters {
            aspect_ratio_info_present_flag,
            aspect_ratio_idc,
            sar_width,
            sar_height,
            overscan_info_present_flag,
            overscan_appropriate_flag,
            video_signal_type_present_flag,
            video_format,
            video_full_range_flag,
            colour_description_present_flag,
            colour_primaries,
            transfer_characteristics,
            matrix_coefficients,
            chroma_loc_info_present_flag,
            timing_info_present_flag,
            num_units_in_tick,
            time_scale,
            fixed_frame_rate_flag
        }
    }

    fn write_vui(vui: &VuiParameters, descriptor_writer: &mut DescriptorWriter) {
        descriptor_writer.append_u1(vui.aspect_ratio_info_present_flag);
        if vui.aspect_ratio_info_present_flag {
            descriptor_writer.append_u8(vui.aspect_ratio_idc);
            if vui.aspect_ratio_idc == 255 {    // Extended_SAR
                descriptor_writer.append_u16(vui.sar_width);
                descriptor_writer.append_u16(vui.sar_height);
            }
        }

        descriptor_writer.append_u1(vui.overscan_info_present_flag);
        if vui.overscan_info_present_flag {
            descriptor_writer.append_u1(vui.overscan_appropriate_flag);
        }

        descriptor_writer.append_u1(vui.video_signal_type_present_flag);
        if vui.video_signal_type_present_flag {
            descriptor_writer.append_u(3, vui.video_format);
            descriptor_writer.append_u1(vui.video_full_range_flag);
            descriptor_writer.append_u1(vui.colour_description_present_flag);
            if vui.colour_description_present_flag {
                descriptor_writer.append_u8(vui.colour_primaries);
                descriptor_writer.append_u8(vui.transfer_characteristics);
                descriptor_writer.append_u8(vui.matrix_coefficients);
            }
        }

        descriptor_writer.append_u1(vui.chroma_loc_info_present_flag);
        if vui.chroma_loc_info_present_flag {
            todo!()
        }

        descriptor_writer.append_u1(vui.timing_info_present_flag);
        if vui.timing_info_present_flag {
            descriptor_writer.append_u32(vui.num_units_in_tick.unwrap());
            descriptor_writer.append_u32(vui.time_scale.unwrap());
            descriptor_writer.append_u1(vui.fixed_frame_rate_flag.unwrap());
        }
    }
}

impl Nalu for SpsNalu {
    fn get_payload_size(&self) -> u32 {
        self.payload_size
    }

    fn write(&self, wtr: &mut File) {
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
                SpsNalu::write_vui(vui_parameters, &mut descriptor_writer);
            },
            None => descriptor_writer.append_u1(false)
        }

        descriptor_writer.append_u(self.residue.0, self.residue.1.into());
        descriptor_writer.append_all(&self.remaining);
        descriptor_writer.write_with_size_and_header(0x67);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl fmt::Display for SpsNalu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let vui_parameters = match &self.vui_parameters {
            Some(vui) => vui.to_string(),
            None => "n/a".to_owned(),
        };
        write!(f, "[SPS(profile_idc={}, level_idc={}, seq_parameter_set_id={}, chroma_format_idc={}, separate_colour_plane_flag={}, pic_width_in_mbs_minus1={}, pic_height_in_map_units_minus1={}, vui_parameters={})]", 
            self.profile_idc, self.level_idc, self.seq_parameter_set_id, self.chroma_format_idc, self.separate_colour_plane_flag, self.pic_width_in_mbs_minus1, self.pic_height_in_map_units_minus1, vui_parameters)
    }
}

impl fmt::Display for VuiParameters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let num_units_in_tick = match self.num_units_in_tick {
            Some(v) => v.to_string(),
            None => String::from("n/a")
        };
        let time_scale = match self.time_scale {
            Some(v) => v.to_string(),
            None => String::from("n/a")
        };
        write!(f, "(num_units_in_tick={}, time_scale={})", num_units_in_tick, time_scale)
    }
}