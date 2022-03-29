use proconio::input;

fn main() {

    input! { 
        a: String,
    }

    let mut result = 0;
    let mut temp = 0;
    for c in a.chars() {
        if c == 'A' || c == 'C' || c == 'G' || c == 'T' {
            temp += 1;
        }else {
            if temp > result {
                result = temp;
            }
            temp = 0;
        }
    }
    if temp > result {
        result = temp;
    }
    println!("{}", result);

}

