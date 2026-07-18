use std::net::TcpListener;
use std::io::prelude::*;
fn main()
{
    let server=TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("boom server is conneted and LIVE on https://127.0.0.1:8080");
    for connection in server.incoming()
    {
        let mut connect=connection.unwrap();
        let mut buffer=[0;1024];
        connect.read(&mut buffer).unwrap();
        let user_info=String::from_utf8_lossy(&buffer[..]);
        if user_info.contains("GET / HTTP/1.1")
        {
             let html_form="HTTP/1.1 200 OK\r\ncontent-type:text/html;charset=utf-8\r\n\r\n
             <h3>Welcome to online calculator</h2><form action='/' method='GET'>
             <input type='text' name='n1' placeholder='number 1'>
             <input type='text' name='n2' placeholder='number 2'>
             <br><br>
             <button type='submit' name='op' value='add'> + </button>
             <button type='submit' name='op' value='sub'> - </button>
             <button type='submit' name='op' value='mul'> * </button>
             <button type='submit' name='op' value='div'> / </button>
             </form>";
             connect.write_all(html_form.as_bytes()).unwrap();
        }
        else if user_info.contains("GET /?n1=")
        {
            let p11=user_info.split("=").nth(1).unwrap_or("0.0");
            let p22=p11.split("&").next().unwrap_or("0.0");
            let n1:f64=p22.trim().parse::<f64>().unwrap_or(0.0);
            let p1=user_info.split("=").nth(2).unwrap_or("0.0");
            let p2=p1.split("&").next().unwrap_or("0.0");
            let n2:f64=p2.trim().parse::<f64>().unwrap_or(0.0);
            let op1=user_info.split("=").nth(3).unwrap_or("add");
            let op2=op1.split(" ").next().unwrap_or("add");
            let op=op2;
            match op
            {
                "add"=>
                {
            let result=format!("={}",n1+n2);
            let r=format!("HTTP/1.1 200 OK\r\ncontent-type:text/html; charset=utf-8\r\n\r\n<h2>after addition result is as below</h2><br><h3>{}</h3><a href='/'>go back</a>",result);
            connect.write_all(r.as_bytes()).unwrap();
                },
                "sub"=>
                {
                    let result=format!("={}",n1-n2);
            let r=format!("HTTP/1.1 200 OK\r\ncontent-type:text/html; charset=utf-8\r\n\r\n<h2>after substraction result is as below</h2><br><h3>{}</h3><a href='/'>go back</a>",result);
            connect.write_all(r.as_bytes()).unwrap();
                },
                "mul"=>
                {
                    let result=format!("={}",n1*n2);
            let r=format!("HTTP/1.1 200 OK\r\ncontent-type:text/html; charset=utf-8\r\n\r\n<h2>after multiplication result is as below</h2><br><h3>{}</h3><a href='/'>go back</a>",result);
            connect.write_all(r.as_bytes()).unwrap();
                },
                "div"=>
                {
                    if n2==0.0
                    {
                        connect.write_all("HTTP/1.1 400 BAD REQUEST\r\ncontent-type:text/html;charset=utf-8\r\n\r\n<h2>second number if zero devision not possible</h2><a href='/'>go back</a>".as_bytes()).expect("failure to response");

                    }
                    else
                    {
                    let result=format!("={}",n1/n2);
            let r=format!("HTTP/1.1 200 OK\r\ncontent-type:text/html; charset=utf-8\r\n\r\n<h2>after division result is as below</h2><br><h3>{}</h3><a href='/'>go back</a>",result);
            connect.write_all(r.as_bytes()).unwrap();
                    }
                }
                _ => 
                {
            let r = "HTTP/1.1 400 BAD REQUEST\r\nContent-Type: text/html\r\n\r\n<h2>Invalid Operator!</h2><a href='/'>Go Back</a>";
            connect.write_all(r.as_bytes()).unwrap();
                }
            }
             
        }
        else
        {
            connect.write_all("HTTP/1.1 404 NOT FOUND\r\ncontent-type:text/html; charset=utf-8\r\n\r\n<h2>page not found</h2><a href='/'>go back</a>".as_bytes()).unwrap();
        }
    }

}