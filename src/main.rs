mod mp4;
mod h264;

use std::fs::File;

fn main() {
    let mut file = File::open("video.mp4").unwrap();
    let atoms = mp4::atom_reader::read_atoms(&mut file, 0);
    for atom in atoms {
        println!("{}", atom);
    }
}
