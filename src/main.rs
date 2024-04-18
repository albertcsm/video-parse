mod mp4;
mod h264;

use std::fs::File;

fn main() {
    let mut in_file = File::open("video.mp4").unwrap();
    let box_list = mp4::box_list::BoxList::read(&mut in_file, 0);
    for atom in &box_list.boxes {
        println!("{}", atom);
    }

    let mut out_file = File::create("clone.mp4").unwrap();
    box_list.write(&mut out_file);
}
