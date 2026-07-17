use std::net::TcpListener;
use std::io::prelude::*;
fn main()
{
    let server=TcpListener::bind("127.0.0.1:8080").expect("port 8080 is busy!");//hayi hi i cant believe this im wring this my dream come true jo mene oplutechinc me dekha tha dusro ko dekhta tha project banate huyi jabki unko code tak nahi pata tha agar unke project me koi bud ya koi word me dikkat a jaati thi to wah usko explain bhi nahi kar paate the but me usko khud apne inn hatho se bana raha hu kash me me poluytechinc me kar paata but im very fullfilled and happy now !!!
    println!("Web server is activated and live on...\nhttps://127.0.0.1:8080");
    for connection in server.incoming()
    {
        let mut connect=connection.expect("Failed to make connection!");
        let mut buffer=[0;1024];
        connect.read(&mut buffer).expect("Failed to take reqwest!");
        connect.write_all("HTTP/1.1 200 OK\r\n\r\n\r\n<h1>Web server is connected and running now on this browser<br>Connection status: Successfull<br>Server owner:Ashish(The Rust MasterMind)<br>Source language:Rust(The Giant)<br>Project no.:My first web PROJECT<br>(Meri Jingadi Ab Shuru Ho Gayi Hai)<h1>".as_bytes()).expect("Failed to response!");
        connect.flush().expect("Failed to flush!");
    }
}

