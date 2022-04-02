use proconio::input;

fn main() {

    input! { 
        _: i32,
        s: String,
    }

    let mut temp = 0;
    let mut x = 0;
    let mut y = 0;
    for i in s.chars() {
        if i == 'R' {
            temp += 1;
            continue;
        }
        if i == 'S' {
            if temp % 4 == 0 {
                x += 1;
            }else if temp % 4 == 1 {
                y -= 1;
            }else if temp % 4 == 2 {
                x -= 1;
            }else if temp % 4 == 3 {
                y += 1;
            }
        }

    }

    println!("{} {}", x, y);

}

