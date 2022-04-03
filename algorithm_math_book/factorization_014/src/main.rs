use proconio::input;

fn main() {

    input! { 
        a: i64,
    }

    let mut v = Vec::new();
    let mut temp = a;
    for i in 2..=a {
        if i * i > a {
            break;
        }
        loop {
            if temp % i == 0 {
                v.push(i);
                temp = temp / i;
            }else{
                break;
            }
        }

    }
    if temp != 1 {
        v.push(temp);
    }

    for (i, d) in v.iter().enumerate() {
        if i == 0 {
            print!("{}", d);
        }else{
            print!(" {}", d);
        }
    }
    println!("");
}

