use std::str::FromStr; //to parse a value from string

pub enum Method{
    GET, //GET(String)
    DELETE, //DELETE(u64)
    POST, //POST=5,
    PUT,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}

impl FromStr for Method{
    type Err = MethodError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s{
            "GET" =>  Ok(Self::GET),
            "DELETE" => Ok(Self::DELETE),
            "POST" => Ok(Self::POST),
            "PUT" => Ok(Self::PUT),
            "HEAD" => Ok(Self::HEAD),
            "CONNECT" => Ok(Self::CONNECT),
            "OPTIONS"=> Ok(Self::OPTIONS),
            "TRACE" => Ok(Self::TRACE),
            "PATCH" => Ok(Self:: PATCH),
            _ => Err(MethodError),
        }
    }
}

pub struct MethodError;
