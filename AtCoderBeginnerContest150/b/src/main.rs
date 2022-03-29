use proconio::input;

fn main() {

    input! { 
        _a: i32,
        b: String,
    }


    println!("{:?}", b.matches("ABC").count());

}

