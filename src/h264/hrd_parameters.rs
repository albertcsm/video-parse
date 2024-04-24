use super::{descriptor_reader::DescriptorReader, descriptor_writer::DescriptorWriter};

#[derive(Debug)]
pub struct HrdParameters {
    pub cpb_cnt_minus1: u64,
    pub bit_rate_scale: u8,
    pub cpb_size_scale: u8,
    pub bit_rate_value_minus1: Vec<u64>,
    pub cpb_size_value_minus1: Vec<u64>,
    pub cbr_flag: Vec<bool>,
    pub initial_cpb_removal_delay_length_minus1: u8,
    pub cpb_removal_delay_length_minus1: u8,
    pub dpb_output_delay_length_minus1: u8,
    pub time_offset_length: u8
}

impl HrdParameters {
    pub fn read(descriptor_reader: &mut DescriptorReader) -> Self {
        let cpb_cnt_minus1 = descriptor_reader.read_ue_v();
        let bit_rate_scale = descriptor_reader.read_u(4).try_into().unwrap();
        let cpb_size_scale = descriptor_reader.read_u(4).try_into().unwrap();
        let mut bit_rate_value_minus1 = vec![];
        let mut cpb_size_value_minus1 = vec![];
        let mut cbr_flag = vec![];
        for _sched_sel_idx in 0..=cpb_cnt_minus1 {
            bit_rate_value_minus1.push(descriptor_reader.read_ue_v());
            cpb_size_value_minus1.push(descriptor_reader.read_ue_v());
            cbr_flag.push(descriptor_reader.read_u1());
        }
        let initial_cpb_removal_delay_length_minus1 = descriptor_reader.read_u(5).try_into().unwrap();
        let cpb_removal_delay_length_minus1 = descriptor_reader.read_u(5).try_into().unwrap();
        let dpb_output_delay_length_minus1 = descriptor_reader.read_u(5).try_into().unwrap();
        let time_offset_length = descriptor_reader.read_u(5).try_into().unwrap();
        HrdParameters {
            cpb_cnt_minus1,
            bit_rate_scale,
            cpb_size_scale,
            bit_rate_value_minus1,
            cpb_size_value_minus1,
            cbr_flag,
            initial_cpb_removal_delay_length_minus1,
            cpb_removal_delay_length_minus1,
            dpb_output_delay_length_minus1,
            time_offset_length
        }
    }

    pub fn write(&self, descriptor_writer: &mut DescriptorWriter) {
        descriptor_writer.append_ue_v(self.cpb_cnt_minus1);
        descriptor_writer.append_u(4, self.bit_rate_scale.into());
        descriptor_writer.append_u(4, self.cpb_size_scale.into());
        for sched_sel_idx in 0..=usize::try_from(self.cpb_cnt_minus1).unwrap() {
            descriptor_writer.append_ue_v(self.bit_rate_value_minus1[sched_sel_idx]);
            descriptor_writer.append_ue_v(self.cpb_size_value_minus1[sched_sel_idx]);
            descriptor_writer.append_u1(self.cbr_flag[sched_sel_idx]);
        }
        descriptor_writer.append_u(5, self.initial_cpb_removal_delay_length_minus1.into());
        descriptor_writer.append_u(5, self.cpb_removal_delay_length_minus1.into());
        descriptor_writer.append_u(5, self.dpb_output_delay_length_minus1.into());
        descriptor_writer.append_u(5, self.time_offset_length.into());
    }
}