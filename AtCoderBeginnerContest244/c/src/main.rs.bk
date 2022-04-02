use std::collections::HashSet;
use std::io::Write;
 
fn main() {
    let n = get_line();
    let n = n.parse::<i32>().unwrap();
 
    let mut set = HashSet::new();
    for v in 1..=(2 * n + 1) {
        set.insert(v);
    }
 
    loop {
        let v = set.iter().next().cloned().unwrap();
        let _ = set.remove(&v);
        println!("{}", v);
        std::io::stdout().flush().unwrap();
 
        let x = get_line();
        let x = x.parse::<i32>().unwrap();
        if x == 0 {
            return;
        }
 
        let _ = set.remove(&x);
    }
}
 
fn get_line() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().to_string()
}
