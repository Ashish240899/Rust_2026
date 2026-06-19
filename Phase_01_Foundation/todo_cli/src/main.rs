use std::io;
fn main()
{
    let mut to_do:Vec<String>=Vec::new();
    loop
    {
    println!("\n\nwhat you want to do\n1-add task\n2-see all task\n3-delete task\n4-close the app\n\n");
    let mut choice=String::new();
    io::stdin().read_line(&mut choice).unwrap();
    let choice=choice.trim();
    match choice
    {
        "1"=>
        {
            println!("\n\nenter task");
            let mut task=String::new();
            io::stdin().read_line(&mut task).unwrap();
            to_do.push(task.trim().to_string());
            println!("\n\ntask has been added");
        }
        "2"=>
        {
            println!("\n\nyou do to list is");
            for (no,task1) in to_do.iter().enumerate()
            {
                println!("{} {}",no+1,task1);
            }
        }
        "3"=>
        {
            println!("\n\nenter the task number\n\n");
            let mut delete=String::new();
            io::stdin().read_line(&mut delete).unwrap();
            let delete:usize=match delete.trim().parse()
            {
                Ok(num)=>num,
                Err(_)=>
                {
                    println!("error:enter numerical value!");
                    continue;
                }
            };
            if delete>0&&delete<=to_do.len()
            {
                let data1=to_do.remove(delete-1);
                println!("\n\n\n{} task has been deleted\n\n\n",data1);
            }
            else
            {
                println!("\nwrong number");
            }
        }
        "4"=>
        {
           println!("\n\napp has been closed!\n");
           break;
        }
        _=>
        {
            println!("\n\nwrong choice\n\n");
        }
    }
}
}