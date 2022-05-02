use proconio::input;

fn main() {

    input! { 
        n: i64,
        s: String,
    }
    
    let mut bf: Vec<i64> = vec![0; 400000 + 1];
    let mut bg: Vec<i64> = vec![0; 400000 + 1];

    for i in 1..=400000 {
        let mut pos: i64 = i; 
        while pos % 3 == 0 {
            pos /= 3;
            bf[i as usize] += 1;
        }
        bg[i as usize] = pos % 3;
    }

    bg[0] = 1;
    for i in 1..=400000 {
        bf[i] += bf[i - 1];
        bg[i] = (bg[i] * bg[i - 1]) % 3;
    }

    let mut ret = 0;
    for (i, c) in s.chars().enumerate() {
        let mut p1 = 0; 
        let p2 = ncr3(&bf, &bg, n as usize - 1, i);
        if c == 'B' {
            p1 = 0;
        }
        if c == 'W' {
            p1 = 1;
        }
        if c == 'R' {
            p1 = 2;
        }
        ret += p1 * p2;
        ret %= 3;
    }

    if n % 2 == 0 {
        ret = (3 - ret) % 3;
    }

    if ret == 0 { 
        println!("B");
    }else if ret == 1 {
        println!("W");
    }else {
        println!("R");
    }

}

fn ncr3(bf: &Vec<i64>, bg: &Vec<i64>, n: usize, r: usize) -> i64 {
    if bf[n] != bf[r] + bf[n - r] {
        return 0;
    }
    if bg[n] == 1 && bg[r] * bg[n - r] == 1 {
        return 1;
    }
	if bg[n] == 1 && bg[r] * bg[n - r] == 2 {
        return 2;
    }
	if bg[n] == 1 && bg[r] * bg[n - r] == 4 {
        return 1;
    }
	if bg[n] == 2 && bg[r] * bg[n - r] == 1 {
        return 2;
    }
	if bg[n] == 2 && bg[r] * bg[n - r] == 2 {
        return 1;
    }
	if bg[n] == 2 && bg[r] * bg[n - r] == 4 {
        return 2;
    }
	return -1;
}

