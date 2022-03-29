use proconio::input;

fn main() {

    input! { 
        a: i32,
    }

    let mut result = 0;
    for i in 3..=a {
        if i % 2 == 0 {
            continue;
        }
        let mut count = 1;
        for j in 1..=((i-1)/2){

            if i % j == 0 {
                count += 1;
            }
        }
        if count == 8 {
            result += 1;
        }
    }


    println!("{}", result);
}

