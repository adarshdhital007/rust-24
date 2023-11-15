// Define the enum 
enum Song {
    WildestDreams,
}

fn main() {
    // Use the enum variant
    let song = Song::WildestDreams;

    // Match against the enum variants
    match song {
        Song::WildestDreams => println!("Wildest Dreams"),
    }
}
