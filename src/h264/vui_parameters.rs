use super::{descriptor_reader::DescriptorReader, descriptor_writer::DescriptorWriter, hrd_parameters::HrdParameters};

#[derive(Debug)]
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
    pub fixed_frame_rate_flag: Option<bool>,
    pub nal_hrd_parameters_present_flag: bool,
    pub nal_hrd_parameters: Option<HrdParameters>,
    pub vcl_hrd_parameters_present_flag: bool,
    pub vcl_hrd_parameters: Option<HrdParameters>,
    pub low_delay_hrd_flag: bool,
    pub pic_struct_present_flag: bool,
    pub bitstream_restriction_flag: bool,
    pub motion_vectors_over_pic_boundaries_flag: bool,
    pub max_bytes_per_pic_denom: u64,
    pub max_bits_per_mb_denom: u64,
    pub log2_max_mv_length_horizontal: u64,
    pub log2_max_mv_length_vertical: u64,
    pub max_num_reorder_frames: u64,
    pub max_dec_frame_buffering: u64
}

impl VuiParameters {
    pub fn read(descriptor_reader: &mut DescriptorReader) -> Self {
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
        let nal_hrd_parameters_present_flag = descriptor_reader.read_u1();
        let mut nal_hrd_parameters = None;
        if nal_hrd_parameters_present_flag {
            nal_hrd_parameters = Some(HrdParameters::read(descriptor_reader));
        }
        let vcl_hrd_parameters_present_flag = descriptor_reader.read_u1();
        let mut vcl_hrd_parameters = None;
        if vcl_hrd_parameters_present_flag{
            vcl_hrd_parameters = Some(HrdParameters::read(descriptor_reader));
        }
        let mut low_delay_hrd_flag = false;
        if nal_hrd_parameters_present_flag || vcl_hrd_parameters_present_flag {
            low_delay_hrd_flag = descriptor_reader.read_u1();
        }
        let pic_struct_present_flag = descriptor_reader.read_u1();
        let bitstream_restriction_flag = descriptor_reader.read_u1();
        let mut motion_vectors_over_pic_boundaries_flag = false;
        let mut max_bytes_per_pic_denom = 0;
        let mut max_bits_per_mb_denom = 0;
        let mut log2_max_mv_length_horizontal = 0;
        let mut log2_max_mv_length_vertical = 0;
        let mut max_num_reorder_frames = 0;
        let mut max_dec_frame_buffering = 0;
        if bitstream_restriction_flag {
            motion_vectors_over_pic_boundaries_flag = descriptor_reader.read_u1();
            max_bytes_per_pic_denom = descriptor_reader.read_ue_v();
            max_bits_per_mb_denom = descriptor_reader.read_ue_v();
            log2_max_mv_length_horizontal = descriptor_reader.read_ue_v();
            log2_max_mv_length_vertical = descriptor_reader.read_ue_v();
            max_num_reorder_frames = descriptor_reader.read_ue_v();
            max_dec_frame_buffering = descriptor_reader.read_ue_v();
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
            fixed_frame_rate_flag,
            nal_hrd_parameters_present_flag,
            nal_hrd_parameters,
            vcl_hrd_parameters_present_flag,
            vcl_hrd_parameters,
            low_delay_hrd_flag,
            pic_struct_present_flag,
            bitstream_restriction_flag,
            motion_vectors_over_pic_boundaries_flag,
            max_bytes_per_pic_denom,
            max_bits_per_mb_denom,
            log2_max_mv_length_horizontal,
            log2_max_mv_length_vertical,
            max_num_reorder_frames,
            max_dec_frame_buffering
        }
    }

    pub fn write(&self, descriptor_writer: &mut DescriptorWriter) {
        descriptor_writer.append_u1(self.aspect_ratio_info_present_flag);
        if self.aspect_ratio_info_present_flag {
            descriptor_writer.append_u8(self.aspect_ratio_idc);
            if self.aspect_ratio_idc == 255 {    // Extended_SAR
                descriptor_writer.append_u16(self.sar_width);
                descriptor_writer.append_u16(self.sar_height);
            }
        }

        descriptor_writer.append_u1(self.overscan_info_present_flag);
        if self.overscan_info_present_flag {
            descriptor_writer.append_u1(self.overscan_appropriate_flag);
        }

        descriptor_writer.append_u1(self.video_signal_type_present_flag);
        if self.video_signal_type_present_flag {
            descriptor_writer.append_u(3, self.video_format);
            descriptor_writer.append_u1(self.video_full_range_flag);
            descriptor_writer.append_u1(self.colour_description_present_flag);
            if self.colour_description_present_flag {
                descriptor_writer.append_u8(self.colour_primaries);
                descriptor_writer.append_u8(self.transfer_characteristics);
                descriptor_writer.append_u8(self.matrix_coefficients);
            }
        }

        descriptor_writer.append_u1(self.chroma_loc_info_present_flag);
        if self.chroma_loc_info_present_flag {
            todo!()
        }

        descriptor_writer.append_u1(self.timing_info_present_flag);
        if self.timing_info_present_flag {
            descriptor_writer.append_u32(self.num_units_in_tick.unwrap());
            descriptor_writer.append_u32(self.time_scale.unwrap());
            descriptor_writer.append_u1(self.fixed_frame_rate_flag.unwrap());
        }

        descriptor_writer.append_u1(self.nal_hrd_parameters_present_flag);
        if self.nal_hrd_parameters_present_flag {
            self.nal_hrd_parameters.as_ref().unwrap().write(descriptor_writer);
        }
        
        descriptor_writer.append_u1(self.vcl_hrd_parameters_present_flag);
        if self.vcl_hrd_parameters_present_flag {
            self.vcl_hrd_parameters.as_ref().unwrap().write(descriptor_writer);
        }
        
        if self.nal_hrd_parameters_present_flag || self.vcl_hrd_parameters_present_flag {
            descriptor_writer.append_u1(self.low_delay_hrd_flag);
        }

        descriptor_writer.append_u1(self.pic_struct_present_flag);
        
        descriptor_writer.append_u1(self.bitstream_restriction_flag);
        if self.bitstream_restriction_flag {
            descriptor_writer.append_u1(self.motion_vectors_over_pic_boundaries_flag);
            descriptor_writer.append_ue_v(self.max_bytes_per_pic_denom);
            descriptor_writer.append_ue_v(self.max_bits_per_mb_denom);
            descriptor_writer.append_ue_v(self.log2_max_mv_length_horizontal);
            descriptor_writer.append_ue_v(self.log2_max_mv_length_vertical);
            descriptor_writer.append_ue_v(self.max_num_reorder_frames);
            descriptor_writer.append_ue_v(self.max_dec_frame_buffering)
        }
    }
}