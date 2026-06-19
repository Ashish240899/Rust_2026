use rand::prelude::*;
use std::io;
use std::cmp::Ordering;
fn main()
{
    println!("\n** WELCOME TO GUESSING GAME **");
    println!("\nBata maine 1 se 100 ke beech kya socha hai?");
    let machine_no=rand::rng().random_range(1..=100);
    loop
    {
         println!("\n\nGuess Daal:");
         let mut user_no=String::new();
         io::stdin().read_line(&mut user_no).unwrap();
         let user_no:u32=match user_no.trim().parse()
         {
            Ok(num)=>num,
            Err(_)=>{
                println!("\nGALAT NUMBER HAI");
                continue;
            }

         };
         match user_no.cmp(&machine_no)
         {
            Ordering::Less=>{println!("Bahut chhota!");}
            Ordering::Greater=>{println!("Bahut bada!");}
            Ordering::Equal=>{println!("YOU WIN! GAME OVER!");break;}
         }
}
}