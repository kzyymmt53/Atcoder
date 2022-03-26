use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32,
    }
    
    let mut v: Vec<i32> = Vec::new();
    v.push(a);
    v.push(b);
    v.push(c);
    v.sort();
   
    for d in v {
        print!("{} ", d);
    }
    println!("");
}
  

