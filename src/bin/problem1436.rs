use std::io;

fn main()
{
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error");
    let find_count: i32 = input.trim().parse().expect("error");
    let mut apo_number:i32 = 666;
    let mut found_apo_number:i32 = 0;
    loop
    {
        if apo_number.to_string().contains("666")
        {
            found_apo_number += 1;
        }
        if find_count == found_apo_number
        {
            println!("{}", apo_number);
            break;
        }
        apo_number += 1;
    }
}