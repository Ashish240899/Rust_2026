use std::io;
use colored::Colorize;
fn main()
{
    let mut ttt=[["1","2","3"],["4","5","6"],["7","8","9"]];
    let mut choice=String::new();
    let mut step=1;
    let mut current_player="X";
    loop
    {
        println!("\n\n");
         for i in 0..3
    {
        println!(" {} | {} | {} ",ttt[i][0],ttt[i][1],ttt[i][2]);
        if i<2
        {
            println!("---|---|---");
        }
    }
        println!("\n\n");
        println!("Player {} enter your task[1-9]:",current_player);
        choice.clear();
        io::stdin().read_line(&mut choice).unwrap();
        let choice:usize=match choice.trim().parse::<usize>()
        {
            Ok(num) if num>=1&&num<=9 => num,
            _=>
            { 
                println!("Please enter a Digit[1-9]");
                continue;
            }
        };
        let row=(choice-1)/3;
        let col=(choice-1)%3;
        if ttt[row][col]=="X"||ttt[row][col]=="O"
        {
            println!("\nThis is already filled");
            continue;
        }
        else
        {
            ttt[row][col]=current_player;
            step+=1;
        }
        if ttt[0][0]==current_player&&ttt[0][1]==current_player&&ttt[0][2]==current_player
        { print_pro(&ttt,current_player,(0,0),(0,1),(0,2)); break;}
        if ttt[1][0]==current_player&&ttt[1][1]==current_player&&ttt[1][2]==current_player
        { print_pro(&ttt,current_player,(1,0),(1,1),(1,2)); break;}
        if ttt[2][0]==current_player&&ttt[2][1]==current_player&&ttt[2][2]==current_player
        { print_pro(&ttt,current_player,(2,0),(2,1),(2,2)); break;}
        if ttt[0][0]==current_player&&ttt[1][1]==current_player&&ttt[2][2]==current_player
        { print_pro(&ttt,current_player,(0,0),(1,1),(2,2)); break;}
        if ttt[0][0]==current_player&&ttt[1][0]==current_player&&ttt[2][0]==current_player
        { print_pro(&ttt,current_player,(0,0),(1,0),(2,0)); break;}
        if ttt[0][1]==current_player&&ttt[1][1]==current_player&&ttt[2][1]==current_player
        { print_pro(&ttt,current_player,(0,1),(1,1),(2,1)); break;}
        if ttt[0][2]==current_player&&ttt[1][2]==current_player&&ttt[2][2]==current_player
        { print_pro(&ttt,current_player,(0,2),(1,2),(2,2));break;}
        if ttt[0][2]==current_player&&ttt[1][1]==current_player&&ttt[2][0]==current_player
        { print_pro(&ttt,current_player,(0,2),(1,1),(2,0)); break;}
         if step==9
        {
            println!("Game is Draw no one Wins");
            break;
        }
        current_player=if current_player=="X"{"O"}else{"X"};
    }
}







fn print_pro(ttt:&[[&str;3];3],current_player:&str,p1:(usize,usize),p2:(usize,usize),p3:(usize,usize))
{
    println!("\n\nPlayer {} you Won the Game!",current_player);
    for r in 0..3
    {
        for c in 0..3
        {
            if (r,c)==p1||(r,c)==p2||(r,c)==p3
            {
                print!(" {} ",ttt[r][c].red());
            }
            else
            {
                print!("{}",ttt[r][c]);
            }
            if c<2
            {
                print!(" | ");
            }

        }
        println!();
        if r<2
        {
            println!(" ---|---|---");
        }
    }
    return
}