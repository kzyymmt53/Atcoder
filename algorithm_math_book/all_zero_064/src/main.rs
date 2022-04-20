use proconio::input;

fn main() {

    input! { 
        n: i64,
        k: i64,
        a: [i64; n],
    }

    let mut sum: i64 = 0;
    for i in a {
        sum += i.abs();
    }

    if sum % 2 != k % 2 {
        println!("No");
    }else{
        if k < sum {
            println!("No");
        }else{
            println!("Yes");
        }
    }

}

