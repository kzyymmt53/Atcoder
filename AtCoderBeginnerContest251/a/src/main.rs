use proconio::input;

fn main() {

    input! { 
        s: String,
    }

    let mut ans: String = "".to_string();

    for _ in 0..6 {
        ans += &s;
        if ans.chars().count() == 6 {
            println!("{}", ans);
            break;
        }
    }

}

