fn main() {

    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
   
    for c in s.chars() {
        if c.is_uppercase() {
            print!("{}", c.to_ascii_lowercase());
        } else {
            print!("{}", c.to_ascii_uppercase());
        }
    }
    println!("");

}
