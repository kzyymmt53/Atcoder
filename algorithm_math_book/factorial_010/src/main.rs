use proconio::input;

fn main() {

    input! { 
        n: i64,
    }


    println!("{}", factorial(n));


}

fn factorial(n: i64) -> i64 {
    if n <= 1 {
        return 1;
    }

    return factorial(n - 1) * n;
}
