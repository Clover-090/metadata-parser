extern crate id3;
use std::io;
use id3::{Tag, TagLike};

fn main() {
    println!("Enter path to music file");

    let mut pathtofile = String::new();

    io::stdin()
        .read_line(&mut pathtofile)
        .expect("Failed to read line");

        if pathtofile.ends_with('\n') {
            pathtofile.pop();
        }
        if pathtofile.ends_with('\r') {
            pathtofile.pop(); 
        }

    let _testpath = String::from("/home/clover/Desktop/music/Dirty Cheap Horse/On The Rob/Cheap Dirty Horse - On the Rob.mp3");

    println!("{}", pathtofile);

    let tag = Tag::read_from_path(pathtofile).expect("Failed to read tag");
    
    if let Some(artist) = tag.artist() {
        println!("artist: {}", artist);
}
}
