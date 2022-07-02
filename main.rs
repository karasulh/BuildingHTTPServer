//1
//HTTP/1.1 Protocol => L7 Protocol, uses TCP, communication via messages instead of stream data.
//SERVER: TCP Listener <=> HTTP Parser <=> Handler
//Server is composed of three of above. First request taken from TCP connection, then HTTP parser parses the message.
//According to message handler is used for function process. These are in a single thread, so it is handling a single request at a time.
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]

mod server; //because server.rs exists outside, we must specify that we will use server module. Then, it takes this module into here.
mod http; //create http/mod.rs to use method and request modules
mod website_handler;

use website_handler::WebsiteHandler;
use server::Server;
use http::Request;//can be used only it with "pub use .." in mod.rs,  otherwise use: //use http::request::Request; 
use http::Method; //use http::method::Method;
use std::env;//to use enviroment variables / to reach paths

fn main(){

    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR")); //env!("CARGO_MANIFEST_DIR") returns the location of Cargo.toml
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path); //If "public path" enviroment variable exists, use it. Else, use default_path
    println!("public path: {}",public_path);
    let server = Server::new("127.0.0.1:8080".to_string()); //write server::Server::new(...) if we didnot write "use server::Server"; 
    server.run(WebsiteHandler::new(public_path));

    //2
    /* 
    //String:
    let string = String::from("127.0.0.1:8080"); //String type string
    let string_slice = &string[10..]; //string slice which is reference to part of our string
    let string_borrow: &str = &string; //auto conversion, string slice refers to our all string
    let string_literal = "1234"; //cannot be changed
    dbg!(&string);
    dbg!(string_slice);
    dbg!(string_borrow);
    dbg!(string_literal);
    //Note: Taking slice of string like [10..0] means that take the bytes after 10. 
    //This is working only for normal ascii characters. It is not working for emoji(4bytes) or different language alphabets.
    //So be careful for that.
    */

    //3
    /*
    //Enumeration:
    let get= Method::GET("abdc".to_string());
    let delete= Method::DELETE(100);
    let post = Method::POST;
    let put= Method::PUT;
    //Note: Enumerations is actually giving a name to numbers to understand easily from the names.
    //The names in enum starts with 0 and continue incrementing 1 if we didnt specify a value. For example GET=0, DELETE=1,PUT=3
    //If we said below POST=5, then the enum becomes like GET=0,DELETE=1,POST=5,PUT=6,HEAD=7.
    //We can use enums with HTTP Codes by assigning correct values.
    //Also enums can take arguments in different types and enum object size is specified in compile time according to the largest argument size.
    */

}

//4
//Get http/request, http/method modules from outside instead of writing them to main file. So take them to comment. I leave them here only for example.
/* 
mod http{
    pub mod request{
        use super::method::Method;
        pub struct Request{
            query_string: Option<String>, 
            method: Method, 
        }
    }
    pub mod method{
        pub enum Method{
            GET, 
        }
    }
}
*/
/*
GET /user?id=10 HTTP/1.1\r\n
HEADERS
BODY
*/





