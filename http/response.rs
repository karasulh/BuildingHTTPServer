use std::fmt::{Display,Formatter,Result as FmtResult};
use std::net::TcpStream;
use std::io::{Write, Result as IoResult};

use super::StatusCode;

#[derive(Debug)] //Response struct can be derived from Debug, because we provides "derive Debug" for StatusCode in status_code.rs
pub struct Response{
    status_code: StatusCode,
    body: Option<String>,
}

impl Response{
    pub fn new(status_code: StatusCode, body:Option<String>) -> Self{
        Response { status_code,body }
    }

    //pub fn send(&self, stream: &mut TcpStream) -> IoResult<()>{ //for TCPStream, it is okay for us but:
    //if we want to generalize send function for all "Write"able object, we can use Write trait as object type
    //if it should be "dynamic dispatch", use "dyn" keyword before Write in function parameter
    //it it should be "static dispatch", use "impl" keyword before Write in function parameter    
    pub fn send(&self, stream: &mut impl Write) -> IoResult<()>{  

        let body = match &self.body{
            Some(b) => b,
            None => "",
        };
        write!( //In this case, we write directly into stream, so no allocation. 
            stream,
            "HTTP/1.1 {} {}\r\n\r\n{}",
            self.status_code,
            self.status_code.reason_phrase(),
            body,
        )
    }
}

impl Display for Response{ //if we use "write!(...)"", this implementation works for Response.
    fn fmt(&self, f: &mut Formatter) -> FmtResult{ 
        let body = match &self.body{
            Some(b) => b,
            None => "",
        };
        write!( //In this case, we write to formatter, then formatter allocates a string which includes things we wrote.
            f,
            "HTTP/1.1 {} {}\r\n\r\n{}",
            self.status_code,
            self.status_code.reason_phrase(),
            body,
        )
    }
}