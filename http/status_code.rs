use std::fmt::{Display,Formatter,Result as FmtResult};

//We can not use Copy trait for heap variable like String, but this enum is like a integer, so we can directly use derive Copy. 
//Thanks to derivation of Copy, it automatically copy the enums, not need to manual copy process.
//However, the compiler doesnot allow only usage of Copy, it wants us also add a Copy trait for derivation. 
#[derive(Copy,Clone,Debug)]
pub enum StatusCode{
    Ok =200,
    BadRequest =400,
    NotFound =404,
}

impl StatusCode {
    pub fn reason_phrase(&self) -> &str {
        match self{
            Self::Ok => "Ok",
            Self::BadRequest => "Bad Request",
            Self::NotFound => "Not Found",
        }
    }
}

impl Display for StatusCode{
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", *self as u16) //we used dereferenced self, because self is a pointer to show enum Statuscode. Also this process needs copy process.
    }
}