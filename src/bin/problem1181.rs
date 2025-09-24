use std::io;

fn main()
{
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error");
    let line_amount:i32 = input.trim().parse().expect("Error");
    let mut string_array = Vec::new();
    for _ in 0..line_amount
    {
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Error");
        string_array.push(input.trim().to_string());
    }
    string_array.sort_by(|a, b| a.len().cmp(&b.len()).then(a.cmp(b)));
    string_array.dedup();
    for i in string_array
    {
        println!("{}", i);
    }
}