#[allow(dead_code)]
use std::io;
enum Operation
{
    Add,Sub,Mul,Div
}
struct Calculator
{
    num1:f64,num2:f64,op:Operation
}
fn calculate(c:Calculator)->f64
{
    match c.op
    {
        Operation::Add=>
        {
            c.num1+c.num2 
        }
        Operation::Sub=>
        {
            c.num1-c.num2 
        }
        Operation::Mul=>
        {
            c.num1*c.num2
        }
        Operation::Div=>
        {
            if c.num2==0.0
            {
                println!("error:second number should not be zero!");
                return 0.0
            }
            else
            {
                c.num1/c.num2
            }
        }
    }
}
fn main()
{
    println!("\n---------WELCOME TO MY FIRST PROJECT CALCULATOR---------\nPlease enter first number:");
    let mut a=String::new();
    io::stdin().read_line(&mut a).unwrap();
    println!("\nPlease enter second number:");
    let mut b=String::new();
    io::stdin().read_line(&mut b).unwrap();
    let number1:f64=match a.trim().parse()
    {
        Ok(num)=>num,
        Err(_)=>
        {
            println!("error:enter only numerical values!");
            return;
        }
    };
    let number2:f64=match b.trim().parse()
    {
        Ok(num)=>num,
        Err(_)=>
        {
            println!("error:enter only numerical values!");
            return;
        }
    };
    println!("\nPlease enter below OPERATIONS\nType + or Add for ADDITION\nType - or Sub for SUBSTRACTION\nType * or Mul for MULTIPLICATION\nType / or Div for DIVISION\n");
    let mut choice=String::new();
    io::stdin().read_line(&mut choice).unwrap();
    let string1=choice.trim();
    let operation=match string1
    {
        "Add"|"+"=>
        {
            Operation::Add 
        }
        "Sub"|"-"=>
        {
            Operation::Sub 
        }
        "Mul"|"*"=>
        {
            Operation::Mul 
        }
        "Div"|"/"=>
        {
            Operation::Div 
        }
        _ =>
        {
            println!("\nOPPS WRONG CHOICE!!\nPlease enter RIGHT ONCES");
            return;
        }
    };
    let c1=Calculator
    {
        num1:number1,num2:number2,op:operation
    };
    let result=calculate(c1);
    println!("\nResult ={}",result);
}