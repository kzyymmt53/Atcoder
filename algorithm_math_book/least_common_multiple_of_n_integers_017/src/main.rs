use proconio::input;

fn main() {

    input! {
        n: usize,
    }

    input! {
        v: [i64; n],
    }

    let mut a = v[0];
    for i in 1..v.len() {
        let b = v[i];
        let temp;
        if a > b {
            temp = common_divisor(a, b);
        }else{
            temp = common_divisor(b, a);
        }
        //割り算の前にbをかけるとオーバフローして誤答になる
        a = a / temp * b;

    }
    println!("{}", a);

}

fn common_divisor(a: i64, b: i64) -> i64 {
    if a % b == 0{
        return b;
    }

    return common_divisor(b, a % b);
}

