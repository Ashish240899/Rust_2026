#[allow(unused_imports)]
use serde::Serialize;
use serde_json;
use std::io::{Read,Write};
use std::net::TcpListener;
use rusqlite::{Connection,params,Result};
#[derive(Serialize)]
struct User
{
    id:i32,
    name:String,
    role:String
}
fn main()
{
    let db=Connection::open("users.db").unwrap();

    db.execute("create table if not exists users (id integer primary key,name text not null, role text not null)",[]).unwrap();

    db.execute("insert or ignore into users (id,name,role) values (?1,?2,?3)", params![1,"Ashish Khattry","RUST developer"]).unwrap();

    let server=TcpListener::bind("127.0.0.1:8080").unwrap();

    for connection in server.incoming()
    {
        let mut connect=connection.unwrap();

        let mut buffer=[0;1024];

        connect.read(&mut buffer).unwrap();

        let user_info=String::from_utf8_lossy(&buffer);

        if user_info.contains("GET /users HTTP/1.1")
        {

            let mut statement =db.prepare("select id, name, role from users").unwrap();
            let user_iterator=statement.query_map([],|row|
            {
                Ok(User
                    {
                id:row.get(0)?,
                name:row.get(1)?,
                role:row.get(2)?,
                    })
            }).unwrap();

            let mut user_list:Vec<User>=Vec::new();

            for user in user_iterator
            {
                user_list.push(user.unwrap());
            }

            let json_data=serde_json::to_string(&user_list).unwrap();

            let response=format!("HTTP/1.1 \r\ncontent-type:application/json; charset=utf-8\r\n\r\n{}",json_data);

              connect.write_all(response.as_bytes()).unwrap();
        }

      

        else
        {

            let error_res="HTTP/1.1 \r\ncontent-type:application/json\r\n\r\n{\"error\":\"route not found\"}";
            connect.write_all(error_res.as_bytes()).unwrap();
        }
    }
}