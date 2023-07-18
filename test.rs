use std::io::Read;
fn main() {
    let path = "/Users/pj/.dev/personal/lapce/lapce/extra/images/logo.png";

    let mut file = std::fs::File::open(path).unwrap();
    // Read the file in as bytes
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    // Parse the file contents as utf8, replacing non-utf8 data with the
    // replacement character
    let contents = String::from_utf8_lossy(&buffer);

    println!("{}", contents.to_string());
}
