use albums::add_album;

fn main() {
    let title = String::from("Dark Side of the Moon");
    let mut album = add_album(title);
    println!("{}", album.title);
    album.set_title(String::from("Meddle"));
    println!("{}", album.title);
}
