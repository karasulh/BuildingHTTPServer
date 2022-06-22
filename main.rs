//HTTP/1.1 Protocol => L7 Protocol, uses TCP, communication via messages instead of stream data.
//SERVER: TCP Listener <=> HTTP Parser <=> Handler
//Server is composed of three of them. First request taken from TCP connection, then HTTP parser parses the message.
//According to message handler is used for function process. These are in a single thread, so it is handling a single request at a time.
fn main(){
    println!("Hello");
}