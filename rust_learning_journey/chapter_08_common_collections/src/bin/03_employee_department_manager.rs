//03_employee_department_manager
use std::collections::HashMap;
use std::io::Write;
fn main()
{
    loop
    {
    print!("Command: ");
    io::stdout().flush().expect("Flushing error!");
    let mut command =String::new();
    std::io::stdin().read_line(&mut command).expect("Input error");
    let command=command.trim();
    let parts:Vec<String>=command.split_whitespace().collect();
    if parts.len()!=4||parts[0].to_lowercase()!="Add"||parts[2].to_lowercase!="to"
    {
        println!("Invalid command!");
        continue;
    }
    let name=parts[1].to_string();
    let dep=parts[3].to_string();
    
    }
}