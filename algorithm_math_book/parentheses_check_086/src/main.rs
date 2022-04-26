use proconio::input;

fn main() {

    input! { 
        _n: usize,
        s: String,
    }

    let mut count: i64 = 0;
    for c in s.chars() {
        if c == '(' {
            count += 1;
        }else{
            count -= 1;
        }

        if count < 0 {
            println!("No");
            return;
        }
    }

    println!("Yes");

}

