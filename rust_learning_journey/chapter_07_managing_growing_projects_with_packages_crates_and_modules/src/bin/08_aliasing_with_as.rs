//08_aliasing_with_as
pub mod local_db
{
    pub fn connect()
    {
        println!("Database connected");
    }
}
pub mod cloud_db
{
    pub fn connect()
    {
        println!("Cloud database conneted");
    }
}
use local_db::connect as LocalConnect;
use cloud_db::connect as CloudConnect;
fn main()
{
    LocalConnect();
    CloudConnect();
}