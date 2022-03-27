fn main() {
    let mut w = String::new();
    std::io::stdin().read_line(&mut w).ok();

    let mut t = String::new();

    loop {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).ok();

    
        if s.trim() == "END_OF_TEXT" {
            break;
        }
        t += &(s.trim().to_owned() + " ");
    
    }
    let v: Vec<&str> = t.split(' ').collect();  
    let mut count = 0;
    for d in v {
        if w.trim() == convert(d) {
            count += 1;
        }
    } 
    println!("{}", count);
}

fn capslock(c: char) -> char {
    if c.is_uppercase() {
        return c.to_ascii_lowercase();
    } 
    return c;
}

fn convert(string: &str) -> String {
    string.chars().map(capslock).collect()
}
