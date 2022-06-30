use std::collections::HashMap;

//a=1&b=2&c&d=&e===&d=7&d=abc //Query String example
pub struct QueryString<'buf>{
    data: HashMap<&'buf str, Value<'buf>>,
}

//It is used to differentiate single value and multiple values(array). For example 'd' in the example use more than one, so it is like an array. 
pub enum Value<'buf>{
    Single(&'buf str), //like 'a' in example
    Multiple(Vec<&'buf str>), //like 'd' in example //Length isnot known at compile time, so use Vector.
}

impl<'buf> QueryString<'buf>{
    pub fn get(&self, key:&str) -> Option<&Value>{ //if there is no key in hashmap, it returns None
        self.data.get(key)
    }
}

//We can not use FromStr because it is not using with types with lifetime, so we will use From.
//a=1&b=2&c&d=&e===&d=7&d=abc //Query String example
impl<'buf> From<&'buf str> for QueryString<'buf>{
    fn from(s: &'buf str) -> Self{
        let mut data = HashMap::new();
        s.split('&');

        QueryString{data}
    }
}