use super::{pps_nalu::PpsNalu, sps_nalu::SpsNalu};

pub trait SpsPpsProvider {
    fn get_pps(&self, id: u64) -> Option<&PpsNalu>;
    fn get_sps(&self, id: u64) -> Option<&SpsNalu>;
}
