use proconio::input;

fn main() {

    input! { 
        mut a: i64,
        b: i64,
        k: i64,
    }

    for i in 0..1000000000 {
        if a >= b {
            println!("{}", i);
            break;
        }

        a *= k;
    }


}

