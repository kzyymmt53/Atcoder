use proconio::input;

fn main() {

    input! { 
        n: usize,
    }

    let mut sum = 0;
    for _ in 0..n {
        input! {
            a: i32,
        }
        sum += a;
    }

    println!("{}", sum);

}

