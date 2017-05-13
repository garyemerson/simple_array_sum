use std::io;

fn main() {
    let n = get_line();
    let n = n.trim().parse::<i32>().unwrap();

    let nums = get_line();
    let nums: Vec<i32> = nums.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect();

    let sum = nums.iter().fold(0, |acc, x| acc + x);
    println!("{}", sum);
}

fn get_line() -> String {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s
}
