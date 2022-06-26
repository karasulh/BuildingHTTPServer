use crate::http::{Request, request}; //crate means root module. In this case root modules is main, and reach http module from main.rs because "mod http" is in main.
use std::net::TcpListener; //for TCP connection
use std::io::Read; //to read stream
use std::convert::TryFrom; //for string and array conversion

pub struct Server{
    addr:String,
}

//Functionality of struct is provided by two way: "methods" and "associated functions"(like 'new' function)
impl Server{
    pub fn new(addr:String) -> Self{ //We can use "Server" instead of Self
        Self { addr }
    }
//not to take ownership of self/this, use &self, so 'run function' only borrows the Server object, 
//so when the "run function" finishes, object will not deallocated.
    pub fn run(self){ //&self
        println!("Listening on {}",self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap(); //because it returns "Result<T,E>", writing unwrap we reach result.
        //But be careful when listener=Error, because the program stops here if we use unwrap instead of match cases.

        loop{//listen the port as being server for new connections
            match listener.accept(){
                Ok((mut stream,_)) => {//return with 2 parameter, one of them is ignored with "_".
                    
                    let mut buffer = [0;1024];
                    match stream.read(&mut buffer){//read the socket and writes content into buffer
                        Ok(_) => {
                            println!("Received a request: {}",String::from_utf8_lossy(&buffer));//with "lossy", the invalid char will not a problem.  
        
                            //Request::try_from(&buffer as &[u8]);//this and below, both way, is okay for usage try_from because compiler wants it as slice. 
                            match Request::try_from(&buffer[..]){
                                Ok(request) => {
                                    /* 
                                    //If we give a lifetime to buffer related to request object, 
                                    //then the compiler can not allow us to change the buffer in here. Because after changing we use request object here again.
                                    //But if we dont give a lifetime to buffer in "TryFrom", the compiler allows us to change buffer here because
                                    //it thinks there is no relation with buffer and request object.
                                    buffer[1]=0;
                                    let a=request;
                                    */
                                },
                                Err(e) => println!("Failed to parse a request: {}",e),
                            }
                            //let res: &Result<Request,_> = &buffer[..].try_into(); //With try_from implementation, automatically opposite case is generated.

                        }
                        Err(e) => println!("Failed to read from connection: {}",e),
                    }
                }
                Err(e) => println!("Failed to establish a connection: {}",e),
            }
            /* //Different use of handling Result type
            let res=listener.accept();//Due to above reason(stopping program) for listener=Error, we will not use unwrap here.We will use match cases.
            if res.is_err(){
                continue; }
            let (stream,addr) = res.unwrap(); //"lister accept result" returns tuple.
            */
        }
        
        /*//5
        //Note:If we want to break outer loop from inside of inner loop, use labels starting with ' like 'outer:
        'outer: loop{
            loop{
                break 'outer;
            }
        }
        */
    }
}