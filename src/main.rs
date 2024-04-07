mod atom;
mod four_cc;
mod ftyp_box;
mod unknown_box;
mod mdat_box;
mod moov_box;
mod mvhd_box;
mod atom_reader;

use std::fs::File;

fn main() {
    let mut file = File::open("video.mp4").unwrap();
    let atoms = atom_reader::read_atoms(&mut file, 0);
    for atom in atoms {
        println!("{}", atom);
    }
}
