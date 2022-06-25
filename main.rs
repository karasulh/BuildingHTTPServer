//HTTP/1.1 Protocol => L7 Protocol, uses TCP, communication via messages instead of stream data.
//SERVER: TCP Listener <=> HTTP Parser <=> Handler
//Server is composed of three of them. First request taken from TCP connection, then HTTP parser parses the message.
//According to message handler is used for function process. These are in a single thread, so it is handling a single request at a time.
#![allow(non_snake_case)]

fn main(){
    
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

    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}

struct Server{
    addr:String,
}

//Functionality of struct is provided by two way: "methods" and "associated functions"(like 'new' function)
impl Server{
    fn new(addr:String) -> Self{ //We can use "Server" instead of Self
        Self { addr }
    }
//not to take ownership of self/this, use &self, so 'run function' only borrows the Server object, 
//so when the "run function" finishes, object will not deallocated.
    fn run(self){ //&self
        println!("Listening on {}",self.addr);
}

}
