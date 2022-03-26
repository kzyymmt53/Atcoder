use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
    }
    
    if a < b {
        println!("{}", "a<b");
    }else if b < a {
        println!("{}", "a>b");
    }else {
        println!("{}", "a==b");
    }
}
