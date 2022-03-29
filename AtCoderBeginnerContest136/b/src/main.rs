use proconio::input;

fn main() {

    input! { 
        a: i32,
    }

    let mut result = 0;
    for i in 1..a+1 {
        if i.to_string().len() % 2 == 1 {
            result += 1;
        }
    }

    println!("{}", result);
}

