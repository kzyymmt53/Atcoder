use proconio::input;

fn main() {

    input! { 
        a: i32,
        b: i32,
        k: i32,
    }

    let n: i32;
    if a <= b {
        n = a;
    }else{
        n = b;
    }

    let mut count = 0;
    for i in (1..=n).rev() {
        if a % i == 0 && b % i == 0 {
            count += 1;
        }
        if count == k {
            println!("{}", i);
            break;
        }
    }

}

