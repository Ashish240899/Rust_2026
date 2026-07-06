use std::io;

fn main() {
    println!("Enter the password:");
    let mut password = String::new();
    io::stdin().read_line(&mut password).unwrap();
    let password = password.trim();
    
    println!("Password is saved.");
    println!("Do you want to see password? (Y/N):");
    
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    
    
    let choice = choice.trim();
    
    if choice == "y" ||choice=="Y"
    {
        println!("= {}", password);
    } else {
        println!("Password hidden.");
    }
}