use proconio::input;

fn main() {
    input! {
        s: i32,
    }
    let hour = s / 3600;
    let minutes = (s - hour * 3600) / 60;
    let seconds = s - hour * 3600 - minutes * 60;
    println!("{}:{}:{}",  hour, minutes, seconds);
}

