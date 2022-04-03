use proconio::input;

fn main() {

    input! { 
        n: i64,
    }

    input! {
        v: [i64; n],
    }

    let mut a = v[0];
    for i in 1..v.len() {
        let b = v[i];
        if a > b {
            a = common_divisor(a, b);
        }else{
            a = common_divisor(b, a);
        }
    }
    println!("{}", a);

}

fn common_divisor(a: i64, b: i64) -> i64 {
    if a % b == 0 {
        return b;
    }

    return common_divisor(b, a % b);
}
