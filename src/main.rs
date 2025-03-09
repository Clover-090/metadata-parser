use id3::Tag;
use std::io;

fn main() {
    println!("Enter path to music file");

    let mut pathtofile= String::new();

    io::stdin()
        .read_line(&mut pathtofile)
        .expect("Failed to read line");

    let tag = Tag::read_from_path(pathtofile).unwrap();

    if let Some(artist) =  tag.artist() {
        println!("artist: {}", artist);
    }
}
