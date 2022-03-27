use std::collections::HashMap;
fn main() {
   
    let mut map = HashMap::new();
    for i in 97..123 {
        let c: char = (i as u8) as char;
        map.insert(c, 0);
    }
    
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
  
    for c in s.chars() {
        let mut c2: char = c;
        if c.is_uppercase() {
            c2 = c.to_ascii_lowercase();
        }
        if !map.contains_key(&c2) {
            continue;
        }
        *map.entry(c2).or_insert(0) += 1;
    }

    for i in 97..123 {
        let a = (i as u8) as char;
        println!("{}:{}", a, map.get(&a).unwrap());
    }
}
