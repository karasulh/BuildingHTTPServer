use super::method::Method;
use std::convert::TryFrom; //use it to conversion of types like array<=>String
use std::error::Error; 
//use std::fmt::Display;
//use std::fmt::Formatter;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult}; //We can show them at the same time with {} 
//Result as FmtResult: to use spesific Result definition inside fmt


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

    //GET /search?name_abc&sort=1 HTTP/1.1
    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        unimplemented!();
        
        /*
        let string=String::from("asd");
        string.encrypt();
        buf.encrypt();
        */
    }
}

pub enum ParseError{ //we create it to show errors of TryFrom implementation as enums instead of string type.
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InavlidMethod,
}
impl ParseError{
    fn message(&self)->&str{
        match self{
           Self::InvalidRequest => "Invalid Request",
           Self::InvalidEncoding => "Invalid Encoding",
           Self::InvalidProtocol => "Invalid Protocol",
           Self::InavlidMethod => "Invalid Method",
        }
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