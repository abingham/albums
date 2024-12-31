use albums::album::add_album;

fn main() {
    let album = add_album("The Dark Side of the Moon".to_string());
    println!("{}", album.title)
}
