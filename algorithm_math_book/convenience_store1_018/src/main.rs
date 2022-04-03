use proconio::input;

fn main() {

    input! { 
        n: i64,
    }

    input! {
        a: [i64; n],
    }

    let mut a100: i64 = 0;
    let mut a200: i64 = 0;
    let mut a300: i64 = 0;
    let mut a400: i64 = 0;

    for d in a {
        if d == 100 {
            a100 += 1;
        }else if d == 200 {
            a200 += 1;
        }else if d == 300 {
            a300 += 1; 
        }else if d == 400{
            a400 += 1;
        }
    }

    let ans: i64 = a100 * a400  + a200 * a300;

    println!("{}", ans);

}

