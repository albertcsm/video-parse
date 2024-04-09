use std::{fmt, io::{self, Read, Seek}};

use super::{descriptor_reader::DescriptorReader, nalu::Nalu};

pub struct SpsNalu {
    pub profile_idc: u8,
    pub level_idc: u8,
    pub seq_parameter_set_id: u64,
    pub chroma_format_idc: u64,
    pub separate_colour_plane_flag: bool,
    pub pic_width_in_mbs_minus1: u64,
    pub pic_height_in_map_units_minus1: u64,
    pub vui_parameters: Option<VuiParameters>,
    pub payload_size: u32
}

pub struct VuiParameters {
    pub num_units_in_tick: Option<u32>,
    pub time_scale: Option<u32>,
    pub fixed_frame_rate_flag: Option<bool>
}

impl SpsNalu {
    pub fn read(rdr: &mut (impl Read + Seek), len: u32) -> io::Result<Self> {
        let mut descriptor_reader = DescriptorReader::new(rdr);
        let profile_idc = descriptor_reader.read_u8();
        descriptor_reader.read_u8();
        let level_idc: u8 = descriptor_reader.read_u8();
        let seq_parameter_set_id = descriptor_reader.read_ue_v();
        let mut chroma_format_idc = 0;
        let mut separate_colour_plane_flag = false;
        match profile_idc {
            100 | 110 | 122 | 244 | 44 | 83 | 86 | 118 | 128 => {
                chroma_format_idc = descriptor_reader.read_ue_v();
                if chroma_format_idc == 3 {
                    separate_colour_plane_flag = descriptor_reader.read_u1();
                }
                let _bit_depth_luma_minus8 = descriptor_reader.read_ue_v();
                let _bit_depth_chroma_minus8 = descriptor_reader.read_ue_v();
                let _qpprime_y_zero_transform_bypass_flag = descriptor_reader.read_u1();
                let seq_scaling_matrix_present_flag = descriptor_reader.read_u1();
                if seq_scaling_matrix_present_flag {
                    for i in 0..=(if chroma_format_idc != 3 { 8 } else { 12 }) {
                        let seq_scaling_list_present_flag = descriptor_reader.read_u1();
                        if seq_scaling_list_present_flag {
                            if i < 6 {
                                todo!()
                            }
                        }
                    }
                }
            }
            _ => {}
        }
        let _log2_max_frame_num_minus4 = descriptor_reader.read_ue_v();
        let pic_order_cnt_type = descriptor_reader.read_ue_v();
        if pic_order_cnt_type == 0 {
            let _log2_max_pic_order_cnt_lsb_minus4 = descriptor_reader.read_ue_v();
        } else if pic_order_cnt_type == 1 {
            let _delta_pic_order_always_zero_flag = descriptor_reader.read_u1();
            todo!()
        }
        let _max_num_ref_frames = descriptor_reader.read_ue_v();
        let _gaps_in_frame_num_value_allowed_flag = descriptor_reader.read_u1();
        let pic_width_in_mbs_minus1 = descriptor_reader.read_ue_v();
        let pic_height_in_map_units_minus1 = descriptor_reader.read_ue_v();
        let frame_mbs_only_flag = descriptor_reader.read_u1();
        if !frame_mbs_only_flag {
            let _mb_adaptive_frame_field_flag = descriptor_reader.read_u1();
        }
        let _direct_8x8_inference_flag = descriptor_reader.read_u1();
        let frame_cropping_flag = descriptor_reader.read_u1();
        if frame_cropping_flag {
            let _frame_crop_left_offset = descriptor_reader.read_ue_v();
            let _frame_crop_right_offset = descriptor_reader.read_ue_v();
            let _frame_crop_top_offset = descriptor_reader.read_ue_v();
            let _frame_crop_bottom_offset = descriptor_reader.read_ue_v();
        }
        let vui_parameters_present_flag = descriptor_reader.read_u1();
        let mut vui_parameters = Option::None;
        if vui_parameters_present_flag {
            vui_parameters = Option::Some(SpsNalu::read_vui(&mut descriptor_reader));
        }

        let read = descriptor_reader.get_num_read_bytes();
        rdr.seek(io::SeekFrom::Current(i64::from(len) - i64::try_from(read).unwrap())).unwrap();
        Ok(SpsNalu {
            profile_idc,
            level_idc,
            seq_parameter_set_id,
            chroma_format_idc,
            separate_colour_plane_flag,
            pic_width_in_mbs_minus1,
            pic_height_in_map_units_minus1,
            vui_parameters,
            payload_size: len
        })
    }

    fn read_vui(descriptor_reader: &mut DescriptorReader) -> VuiParameters {
        let aspect_ratio_info_present_flag = descriptor_reader.read_u1();
        if aspect_ratio_info_present_flag {
            let aspect_ratio_idc = descriptor_reader.read_u8();
            if aspect_ratio_idc == 255 {    // Extended_SAR
                let _sar_width = descriptor_reader.read_u16();
                let _sar_height = descriptor_reader.read_u16();
            }
        }
        let overscan_info_present_flag = descriptor_reader.read_u1();
        if overscan_info_present_flag {
            let _overscan_appropriate_flag = descriptor_reader.read_u1();
        }
        let video_signal_type_present_flag = descriptor_reader.read_u1();
        if video_signal_type_present_flag {
            let _video_format = descriptor_reader.read_u(3);
            let _video_full_range_flag = descriptor_reader.read_u1();
            let colour_description_present_flag = descriptor_reader.read_u1();
            if colour_description_present_flag {
                let _colour_primaries = descriptor_reader.read_u8();
                let _transfer_characteristics = descriptor_reader.read_u8();
                let _matrix_coefficients = descriptor_reader.read_u8();
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
            num_units_in_tick,
            time_scale,
            fixed_frame_rate_flag
        }
    }
}

impl Nalu for SpsNalu {
    fn get_payload_size(&self) -> u32 {
        self.payload_size
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