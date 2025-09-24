use std::io;

fn main()
{
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error");
    let bytes = input.trim().as_bytes();
    let mut sum = 0;
    let mut missing_number:i32;
    let mut missing_point = 0;
    let check_digit = (bytes[12] - b'0') as i32;
    let mut value:i32;
    for (i, &input)  in bytes.iter().take(12).enumerate()
    {
        if input == b'*'
        {
            missing_point = i;
            continue;
        }
        value = (input -b'0') as i32;
        if (i+1) % 2 == 0
        {
            sum += (value*3);
        }
        else
        {
            sum += value;
        }
    }
    for n in 0..=9
    {
        let mut current_sum = sum;
        if(missing_point + 1) %2 == 0
        {
            missing_number = n*3;
        }
        else
        {
            missing_number = n;
        }
        current_sum += missing_number;
        if(current_sum + check_digit) %10 == 0
        {
            println!("{}", n);
            return;
        }
    }
}