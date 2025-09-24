use std::io;

fn main()
{
    let mut input = String::new();
    let mut input_number:i32 = 0;
    for _ in 0..3
    {
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Error");
        match input.trim().parse::<i32>()
        {
            Ok(num) =>
                {
                    input_number = num;
                }
            Err(_) =>
                {
                    if input_number > 0
                    {
                        input_number += 1;
                    }
                }
        }
    }
    if (input_number+1) %3 == 0 && (input_number+1) % 5 == 0
    {
        println!("FizzBuzz");
    }
    else if (input_number+1) % 3 == 0 && (input_number+1) % 5 != 0
    {
        println!("Fizz");
    }
    else if (input_number+1) % 3 != 0 && (input_number+1) % 5 == 0
    {
        println!("Buzz");
    }
    else
    {
        println!("{}", (input_number+1));
    }
}