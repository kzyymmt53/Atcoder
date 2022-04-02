use proconio::input;

fn main() {

    input! { 
        _a: i32,
        s: String,
    }

    println!("{}", s.chars().nth(s.chars().count() - 1).unwrap());


}

