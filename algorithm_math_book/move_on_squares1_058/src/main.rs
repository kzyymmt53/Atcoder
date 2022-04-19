use proconio::input;

fn main() {

    input! { 
        n: i64,
        x: i64,
        y: i64,
    }

    if n % 2 != (x + y) % 2 {
        println!("No");
        return;
    }

    if n < x.abs() + y.abs() {
        println!("No");
        return;
    }

    println!("Yes");



}

