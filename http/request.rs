use super::method::{Method,MethodError};
use std::convert::TryFrom; //use it to conversion of types like array<=>String
use std::error::Error; 
//use std::fmt::Display;
//use std::fmt::Formatter;
//Result as FmtResult: to use spesific Result definition inside fmt
use std::fmt::{Debug, Display, Formatter, Result as FmtResult}; //We can show them at the same time with {} 
use std::str; //to convert slice to &str
use std::str::Utf8Error; //"?" will converts utf-8 error to another error, so we implement this.


pub struct Request{
    path: String,
    query_string: Option<String>, //Use Option for the case query_string is empty, Option makes it safe. If there is no String, it returns None.
    method: Method, //super::method::Method //Use "super" to reach high level module, like http in this case.
}

impl Request{
    fn from_byte_array(buf: &[u8]) -> Result<Self,String>{
        unimplemented!();
    }
}

impl TryFrom<&[u8]> for Request{ //When we implement this, automatically compiler implements try_into trait for opposite case.
    type Error = ParseError;

    //GET /search?name_abc&sort=1 HTTP/1.1\r\n...HEADERS...  <= Our Aim: To parse this.
    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        /*
        match str::from_utf8(buf){
            Ok(request) => {}
            Err(_) => return Err(ParseError::InvalidEncoding),
        }
        match str::from_utf8(buf).or(Err(ParseError::InvalidEncoding)){ //It has the same functionality with above 
            Ok(request)=>{}
            Err(e)=>return Err(e),
        }
        //It is also another alternative usage above. "?" tries to convert the error inside of it into type of Error of trait.
        //In this case, there isnot anything to do, because the "type Error" and the error inside of argument are the same which is ParseError type.
        let request= str::from_utf8(buf).or(Err(ParseError::InvalidEncoding))?; //It is okay without anything
        */
        //But if we use like that, this question mark tries to convert utf-8 error message to ParseError, so we will implement it with From<Utf8Error>
        let request = str::from_utf8(buf)?; //We must impl From<Utf8Error> for ParseError to use it.
        
        /*//This method is missing because, we should return Result(Ok) not Option(Some),so use the below method
        match get_next_word(request){
            Some((method,request)) => {}
            None => return Err(ParseError::InvalidRequest),
        }
        */
        ////GET /search?name_abc&sort=1 HTTP/1.1\r\n...HEADERS...  <= Our Aim: To parse this.
        let (method, request)= get_next_word(request).ok_or(ParseError::InvalidRequest)?;//It will also convert Option to Result
        let (path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol,_) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        //It converts from string to something //we implement FromStr for Method and From for MethodError(so, it automatically converts Error to MethodError.).
        let method:Method = method.parse()? ; //But we must specify the target conversion type "Method"

        unimplemented!();
    }
}

fn get_next_word(request: &str) -> Option<(&str,&str)>{//to parse message, returns tuple Option
    for(i,c) in request.chars().enumerate(){
        if c==' ' || c=='\r' {
            return Some((&request[..i],&request[i+1..]));
        }
    }
    None
}

pub enum ParseError{ //we create it to show errors of TryFrom implementation as enums instead of string type.
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}
impl ParseError{
    fn message(&self)->&str{
        match self{
           Self::InvalidRequest => "Invalid Request",
           Self::InvalidEncoding => "Invalid Encoding",
           Self::InvalidProtocol => "Invalid Protocol",
           Self::InvalidMethod => "Invalid Method",
        }
    }
}

impl From<MethodError> for ParseError{ // for method.parse 
    fn from(_: MethodError) -> Self{
        Self::InvalidMethod
    }
}

impl From<Utf8Error> for ParseError{
    fn from(_: Utf8Error) -> Self{
        Self::InvalidEncoding
    }
}

impl Display for ParseError{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult{
        write!(f,"{}",self.message())
    }
}
impl Debug for ParseError{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult{
        write!(f,"{}",self.message())
    }
}
impl Error for ParseError{ //Error trait can be used for traits which must be a implementation of Debug and Display traits.

}


//6
/* //We can use traits and impl like that
trait Encrypt{
    fn encrypt(&self)->Self;
}
impl Encrypt for String{
    fn encrypt(&self)->Self{
        unimplemented!();
    }
}
impl Encrypt for &[u8]{
    fn encrypt(&self)->Self {
        unimplemented!();
    }
}
*/