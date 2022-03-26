use proconio::input;

fn main() {
    input! {
        h: i32,
        m: i32,
        h2: i32,
        m2: i32,
    }
    let takahasi = h * 3600 + m * 60;
    let aoki = h2 * 3600 + m2 * 60 + 1;

    if takahasi < aoki {
        println!("Takahashi");
    }else {
        println!("Aoki");
    }
}

