use proconio::input;

fn main() {
    input! {
        n: usize,
        s: String,
    }
   
    let schars: Vec<char> = s.chars().collect();

    let mut result = String::from("");
    let mut l = 0;
    let mut r = n - 1;

    while l <= r {
        let mut check = false;
        for i in 0..(r - l) {
            if schars[l + i] < schars[r - i] {
                check = true; 
                break;
            }
            if schars[l + i] > schars[r - i] {
                check = false; 
                break;
            }
        }
        if check {
            result.push(schars[l]);
            l += 1;
        }else{
            result.push(schars[r]);
            r -= 1;
        }
    }
    println!("{}", result);
}
