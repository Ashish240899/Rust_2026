use std::io;
fn main()
{
    let mut board:[[char;3];3]=[['1','2','3'],['4','5','6'],['7','8','9']];
    let mut won=false;
    let mut current_player='X';
    let mut input=String::new();
    let mut step=0;
    println!("\n---<<>> WELCOME TO TIK TAK TO CREATED BY ASHISH <<>>---\n");
    loop
    {
        for i in 0..=2
        {
            println!(" {} | {} | {} ",board[i][0],board[i][1],board[i][2]);
            if i<2
            {
                println!("---|---|---");
            }
        }
        println!("\nplayer {} choice your box 1-9 :",current_player);
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let input:usize= match input.trim().parse()
        {
            Ok(no) if no>=1&&no<=9=>no,
            _=>
            {
                println!("error: choice only box 1-9!");
                continue;
            }
        };
        let row=(input-1)/3;
        let col=(input-1)%3;
        if board[row][col]=='X'||board[row][col]=='O'
        {
            println!("error: this box is already full choice anather box!");
            continue;
        }
        board[row][col]=current_player;
        for i in 0..=2
        {
            if board[i][0]==current_player&&board[i][1]==current_player&&board[i][2]==current_player
            {
                won=true;
            }
            if board[0][i]==current_player&&board[1][i]==current_player&&board[2][i]==current_player
            {
                won=true;
            }
        }
        if board[0][0]==current_player&&board[1][1]==current_player&&board[2][2]==current_player
        {
            won=true;
        }
        if board[2][0]==current_player&&board[1][1]==current_player&&board[0][2]==current_player
        {
            won=true;
        }
        if won==true
        {
            println!("\nsuccessfull: congratulation player {} you won the game !\n\n",current_player);
            for i in 0..=2
            {
                println!(" {} | {} | {} ",board[i][0],board[i][1],board[i][2]);
                if i<2
                {
                    println!("---|---|---");
                }
            }
            println!("\n\n");
            break;
        }
        step+=1;
        if step==9
        {
            println!("error: no one wins! the game is draw!");
            break;
        }
        current_player=if current_player=='X'{'O'} else {'X'};
    }
}