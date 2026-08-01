//04_banking_enum_method
#[derive(Debug)]
#[allow(dead_code)]
enum Trasaction
{
    Deposit(u32),
    Withdraw(u32),
}
impl Trasaction
{
    fn process(&self)
    {
        println!("Process trasaction");
    }
}
fn main()
{
    let d=Trasaction::Deposit(5_00_000);
    let w=Trasaction::Withdraw(25_000);
    d.process();
    w.process();
    println!("Deposit ={:?} withdraw={:?}",d,w);
}