use proconio::input;

fn main() {

    input! { 
        n: i64,
    }

    if n % 2 != 0 {
        return;
    }

    let mut ans: Vec<String> = Vec::new();
    for i in 0..(1 << n){
        let mut right = 0;
        let mut left = 0;
        let mut s: String = "".to_string();
        for j in 0..n {
            if i & (1 << j) != 0 {
                left += 1;
                s += "(";
            }else{
                if j == 0 {
                    break;
                }
                right += 1;
                if left - right < 0 {
                    break;
                }
                s += ")";
            }
        }
        if right == left && right + left == n {
            ans.push(s); 
        }
    }

    ans.sort();
    for i in ans{
        println!("{}", i);
    }
}


