use super::method::Method;
use std::convert::TryFrom; //use it to conversion of types like array<=>String
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
    type Error = String;

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        unimplemented!();
        
        /*
        let string=String::from("asd");
        string.encrypt();
        buf.encrypt();
        */
    }
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