use proconio::input;

fn main() {

    input! { 
        a: i64,
        b: i64,
    }

    if a > b {
        println!("{}", common_divisor(a, b));
    }else{
        println!("{}", common_divisor(b, a));
    }

}

fn common_divisor(a: i64, b: i64) -> i64 {
    if a % b == 0 {
        return b;
    }

    return common_divisor(b, a % b);
}

