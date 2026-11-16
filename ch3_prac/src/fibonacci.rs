use std::io;

fn sum_to(n: u32) -> u64 {
    if n <= 2 {
        return 1;
    }
    
    let mut i = 0;
    let mut now = 1;
    let mut prev = 1;
    while i < n - 2 {
        let tmp = now;
        now += prev;
        prev = tmp;
        i += 1;
    }

    return now;
}

pub fn cal() {
    println!("input the n fibonacci you want");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("read error");

    let n: u32 = input.trim().parse().expect("not right number");
    let ans: u64 = sum_to(n);

    println!("the {} fibonacci is: {}", n, ans);
}
