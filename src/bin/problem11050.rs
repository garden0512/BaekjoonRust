use std::io::{self, BufRead};

fn main()
{
    let mut input = String::new();
    let mut reader = io::stdin().lock();
    reader
        .read_line(&mut input)
        .expect("Error");
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let a:i32 = input.next().expect("Error");
    let b:i32 = input.next().expect("Error");
    let n:i32;
    let k:i32;
    if a<b
    {
        n = b;
        k = a;
    }
    else
    {
        n = a;
        k = b;
    }
    let nck = factorial(n)/(factorial(n-k) * factorial(k));
    println!("{}", nck);
}

fn factorial(n:i32) -> i32
{
    if n==0||n==1
    {
        return 1;
    }
    else
    {
        return n*factorial(n-1);
    }
}