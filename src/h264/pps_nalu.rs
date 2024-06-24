use std::{any::Any, fmt, io::{self, Read, Write}};

use super::{descriptor_reader::DescriptorReader, descriptor_writer::DescriptorWriter, nalu::Nalu, sps_pps_provider::SpsPpsProvider};

#[derive(Debug, Clone)]
pub struct PpsNalu {
    pub pic_parameter_set_id: u64,
    pub seq_parameter_set_id: u64,
    pub entropy_coding_mode_flag: bool,
    pub bottom_field_pic_order_in_frame_present_flag: bool,
    pub num_slice_groups_minus1: u64,
    pub num_ref_idx_10_default_active_minus1: u64,
    pub num_ref_idx_l1_default_active_minus1: u64,
    pub weighted_pred_flag: bool,
    pub weighted_bipred_idc: u8,
    pub pic_init_qp_minus26: i64,
    pub pic_init_qs_minus26: i64,
    pub chroma_qp_index_offset: i64,
    pub deblocking_filter_control_present_flag: bool,
    pub constrained_intra_pred_flag: bool,
    pub redundant_pic_cnt_present_flag: bool,
    pub payload_size: u32
}

impl PpsNalu {
    pub fn read(rdr: &mut impl Read, len: u32) -> io::Result<Self> {
        let mut descriptor_reader = DescriptorReader::new(rdr, len);
        let pic_parameter_set_id = descriptor_reader.read_ue_v();
        let seq_parameter_set_id = descriptor_reader.read_ue_v();
        let entropy_coding_mode_flag = descriptor_reader.read_u1();
        let bottom_field_pic_order_in_frame_present_flag = descriptor_reader.read_u1();
        let num_slice_groups_minus1 = descriptor_reader.read_ue_v();
        if num_slice_groups_minus1 > 0 {
            todo!("num_slice_groups_minus1 > 0")
        }
        let num_ref_idx_10_default_active_minus1 = descriptor_reader.read_ue_v();
        let num_ref_idx_l1_default_active_minus1 = descriptor_reader.read_ue_v();
        let weighted_pred_flag = descriptor_reader.read_u1();
        let weighted_bipred_idc = u8::try_from(descriptor_reader.read_u(2)).unwrap();
        let pic_init_qp_minus26 = descriptor_reader.read_se_v();
        let pic_init_qs_minus26 = descriptor_reader.read_se_v();
        let chroma_qp_index_offset = descriptor_reader.read_se_v();
        let deblocking_filter_control_present_flag = descriptor_reader.read_u1();
        let constrained_intra_pred_flag = descriptor_reader.read_u1();
        let redundant_pic_cnt_present_flag = descriptor_reader.read_u1();
        
        if descriptor_reader.more_rbsp_data() {
            todo!("more rbsp data in pps");
        }

        descriptor_reader.read_rbsp_trailing_bits();

        Ok(PpsNalu {
            pic_parameter_set_id,
            seq_parameter_set_id,
            entropy_coding_mode_flag,
            bottom_field_pic_order_in_frame_present_flag,
            num_slice_groups_minus1,
            num_ref_idx_10_default_active_minus1,
            num_ref_idx_l1_default_active_minus1,
            weighted_pred_flag,
            weighted_bipred_idc,
            pic_init_qp_minus26,
            pic_init_qs_minus26,
            chroma_qp_index_offset,
            deblocking_filter_control_present_flag,
            constrained_intra_pred_flag,
            redundant_pic_cnt_present_flag,
            payload_size: len
        })
    }
}

impl Nalu for PpsNalu {
    fn write(&self, wtr: &mut dyn Write, _sps_pps_provider: &dyn SpsPpsProvider) {
        let mut descriptor_writer = DescriptorWriter::new(wtr);
        descriptor_writer.append_ue_v(self.pic_parameter_set_id);
        descriptor_writer.append_ue_v(self.seq_parameter_set_id);
        descriptor_writer.append_u1(self.entropy_coding_mode_flag);
        descriptor_writer.append_u1(self.bottom_field_pic_order_in_frame_present_flag);
        descriptor_writer.append_ue_v(self.num_slice_groups_minus1);
        if self.num_slice_groups_minus1 > 0 {
            todo!("num_slice_groups_minus1 > 0")
        }
        descriptor_writer.append_ue_v(self.num_ref_idx_10_default_active_minus1);
        descriptor_writer.append_ue_v(self.num_ref_idx_l1_default_active_minus1);
        descriptor_writer.append_u1(self.weighted_pred_flag);
        descriptor_writer.append_u(2, self.weighted_bipred_idc.into());
        descriptor_writer.append_se_v(self.pic_init_qp_minus26);
        descriptor_writer.append_se_v(self.pic_init_qs_minus26);
        descriptor_writer.append_se_v(self.chroma_qp_index_offset);
        descriptor_writer.append_u1(self.deblocking_filter_control_present_flag);
        descriptor_writer.append_u1(self.constrained_intra_pred_flag);
        descriptor_writer.append_u1(self.redundant_pic_cnt_present_flag);
        descriptor_writer.append_rbsp_trailing_bits();
        descriptor_writer.write_with_header(0x68);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl fmt::Display for PpsNalu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[PPS(pic_parameter_set_id={}, seq_parameter_set_id={})]", self.pic_parameter_set_id, self.seq_parameter_set_id)
    }
}
