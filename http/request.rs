use super::method::{Method,MethodError};
use std::convert::TryFrom; //use it to conversion of types like array<=>String
use std::error::Error; 
//use std::fmt::Display;
//use std::fmt::Formatter;
//Result as FmtResult: to use spesific Result definition inside fmt
use std::fmt::{Debug, Display, Formatter, Result as FmtResult}; //We can show them at the same time with {} 
use std::str; //to convert slice to &str
use std::str::Utf8Error; //"?" will converts utf-8 error to another error, so we implement this.
use super::{QueryString};//define queryString struct instead of basic string

//We will use a lifetime 'buf for the slices because, when the some functions are finished, array(buffer) will be deallocated but we will need this.
//To prevent deallocation, we give slices a lifetime. Their lifetime are the same with Request object's lifetime. 
#[derive(Debug)]
pub struct Request<'buf>{
    path: &'buf str, //not to use heap, we use &str with lifetime not String.
    query_string: Option<QueryString<'buf>>, //Use Option for the case query_string is empty, Option makes it safe. If there is no String, it returns None.
    method: Method, //super::method::Method //Use "super" to reach high level module, like http in this case.
}

impl<'buf> Request<'buf>{
    
    //For getter functions, return a reference of the arguments
    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn query_string(&self)-> Option<&QueryString>{ //we are interested QueryString in the Option as return value, so use this trick:
        self.query_string.as_ref() //as_ref: "&Option<QeuryString> -> Option<&QueryString>"
    }
    
    fn from_byte_array(buf: &[u8]) -> Result<Self,String>{
        unimplemented!();
    }
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> { //When we implement this, automatically compiler implements try_into trait for opposite case.
    type Error = ParseError;

    //GET /search?name_abc&sort=1 HTTP/1.1\r\n...HEADERS...  <= Our Aim: To parse this.
    //8
    //We know from the Request struct lifetime so, return is: Result<Request<'buf>, Self::Error>
    //However, compiler initially thinks like below, so there will be some contradiction between lifetime 'a and 'buf. To prevent this, we must say
    //the arguments of try_from's buf's lifetime is the same as Request lifetime, 'buf.
    //fn try_from<'a>(buf: &'a [u8]) -> Result<Request<'buf>, Self::Error> { }
    //Solve this like that:
    fn try_from(buf: &'buf [u8]) -> Result<Self, Self::Error> { // Result<Request<'buf>, Self::Error> 

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
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol,_) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        //It converts from string to something //we implement FromStr for Method and From for MethodError(so, it automatically converts Error to MethodError.).
        let method:Method = method.parse()? ; //But we must specify the target conversion type "Method"

        let mut query_string = None;
        
        //Parsing path string can be done with three way:
        /*
        match path.find('?') { //first way
            Some(i) => {
                query_string = Some(&path[i+1..]);
                path = &path[..i]; //take real path like "search" in GET /search?name...
            }
            None => {}
        }

        let q=path.find('?'); //second way without writing "None => {}"
        if q.is_some(){
            let i=q.unwrap();
            query_string=Some(&path[i+1..]);
            path = &path[..i];
        }
        */
        if let Some(i) = path.find('?'){ //third way with "if let" //beatiful and shortest one
            query_string = Some(QueryString::from(&path[i+1..])); //Conversion from string to QueryString with parse process
            path = &path[..i];
        }
        
        Ok(Self{
            path,
            query_string,
            method,
        })
        
    }
}

//7
//If we have two arguments for this function, we must define a lifetime for each them, but if we have only one variable like in the real case, 
//the compiler can understand the return objects lifetime is equal to one argument's lifetime.
//fn get_next_word<'a,'b>(request: &'a str, b: &'b str ) -> Option<(&'a str,&'a str)>

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