use proconio::input;

fn main() {

    input! { 
        n: usize,
    }

    let mut v = vec![];

    for _ in 0..n {
        input! {
            a: [f64; 2],
        }
        v.push(a);
    }


    let mut order: Vec<usize> = Vec::new();

    for i in 0..n {
        order.push(i);
    }

    let mut ans = 0.0;
    let mut count = 0.0;
    while  {
        count += 1.0;
        for i in 1..n {
            ans += ((v[order[i - 1]][0] - v[order[i]][0]).powf(2.0) + (v[order[i - 1]][1] - v[order[i]][1]).powf(2.0)).powf(0.5);
        }
        order.next_permutation()
    }{}

    ans = ans / count;

    println!("{:.10}", ans);

    
    

}


pub trait LexicalPermutation { 
    fn next_permutation(&mut self) -> bool; 
    fn prev_permutation(&mut self) -> bool; 
}

impl<T> LexicalPermutation for [T] where T: Ord {
    fn next_permutation(&mut self) -> bool {
        if self.len() < 2 { return false; }
        let mut i = self.len() - 1;

        while i > 0 && self[i-1] >= self[i] {
            i -= 1;
        }

        if i == 0 {
            return false;
        }

        let mut j = self.len() - 1;

        while j >= i && self[j] <= self[i-1]  {
            j -= 1;
        }

        self.swap(j, i-1); 
        self[i..].reverse();  

        true 

    }

    fn prev_permutation(&mut self) -> bool {
        if self.len() < 2 { return false; }
        let mut i = self.len() - 1;
        while i > 0 && self[i-1] <= self[i] {
            i -= 1;
        }

        if i == 0 { 
           return false;
        }

        self[i..].reverse();

        let mut j = self.len() - 1;
        while j >= i && self[j-1] < self[i-1]  {
            j -= 1;
        }

        self.swap(i-1, j);

        true

    }

}




