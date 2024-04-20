mod mp4;
mod h264;

use std::fs::File;

use h264::{pps_nalu::PpsNalu, sps_nalu::SpsNalu, sps_pps_provider::SpsPpsProvider};
use mp4::mdat_box::MdatBox;

use crate::h264::{idr_nalu::IdrNalu, non_idr_nalu::NonIdrNalu};

fn get_pic_order_cnt(pic_order_cnt_lsb: u32, last_pic_order_cnt: u32, pic_order_cnt_bits: u8) -> u32 {
    let candidate0: u32;
    if last_pic_order_cnt >> 5 >= 1 {
        candidate0 = (((last_pic_order_cnt >> 5) - 1) << 5) | pic_order_cnt_lsb;
    } else {
        candidate0 = u32::MAX;
    }
    let candidate1 = ((last_pic_order_cnt >> 5) << 5) | pic_order_cnt_lsb;
    let candidate2 = (((last_pic_order_cnt >> 5) + 1) << 5) | pic_order_cnt_lsb;
    let diff0 = (candidate0 as i64 - last_pic_order_cnt as i64).abs();
    let diff1 = (candidate1 as i64 - last_pic_order_cnt as i64).abs();
    let diff2 = (candidate2 as i64 - last_pic_order_cnt as i64).abs();
    if diff0 <= diff1 && diff0 <= diff2 {
        candidate0
    } else if diff1 <= diff0 && diff1 <= diff2 {
        candidate1
    } else {
        candidate2
    }
}

fn main() {
    let mut in_file = File::open("video.mp4").unwrap();
    let mut box_list = mp4::box_list::BoxList::read(&mut in_file, 0);
    
    let mut pps: &PpsNalu;
    let mut sps: &SpsNalu;
    let mut pic_order_cnt_bits = None;
    let mut pic_order_cnt = 0;
    let mut frame_num = 0;
    for atom in &mut box_list.boxes {
        if let Some(mdat) = atom.as_any().downcast_ref::<MdatBox>() {
            for nalu in mdat.nalu_list.get_units() {
                if let Some(idr) = nalu.as_any().downcast_ref::<IdrNalu>() {
                    pps = mdat.nalu_list.get_pps(idr.slice_header.pic_parameter_set_id).unwrap();
                    sps = mdat.nalu_list.get_sps(pps.seq_parameter_set_id).unwrap();
                    pic_order_cnt_bits = Some(u8::try_from(sps.log2_max_pic_order_cnt_lsb_minus4 + 4).unwrap());
                    pic_order_cnt = get_pic_order_cnt(idr.slice_header.pic_order_cnt_lsb.try_into().unwrap(), pic_order_cnt, pic_order_cnt_bits.unwrap());
                    frame_num = 0;
                    println!("IDR({}): frame_num={}({}) pic_order={}({})",
                        idr.slice_header.slice_type,
                        idr.slice_header.frame_num, frame_num,
                        idr.slice_header.pic_order_cnt_lsb, pic_order_cnt);                    
                } else if let Some(non_idr) = nalu.as_any().downcast_ref::<NonIdrNalu>() {
                    pic_order_cnt = get_pic_order_cnt(non_idr.slice_header.pic_order_cnt_lsb.try_into().unwrap(), pic_order_cnt, pic_order_cnt_bits.unwrap());
                    if non_idr.slice_header.slice_type == 5 {
                        frame_num += 1;
                    }
                    println!("Non-IDR({}): frame_num={}({}) pic_order={}({})",
                    non_idr.slice_header.slice_type,
                    non_idr.slice_header.frame_num, frame_num,
                    non_idr.slice_header.pic_order_cnt_lsb, pic_order_cnt);
                }
            }
        }
        // println!("{}", atom);
    }

    let mut out_file = File::create("clone.mp4").unwrap();
    box_list.write(&mut out_file);
}
