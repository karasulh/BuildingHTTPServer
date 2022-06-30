use std::collections::HashMap;

//a=1&b=2&c&d=&e===&d=7&d=abc //Query String example
#[derive(Debug)]
pub struct QueryString<'buf>{
    data: HashMap<&'buf str, Value<'buf>>,
}

//It is used to differentiate single value and multiple values(array). For example 'd' in the example use more than one, so it is like an array. 
#[derive(Debug)]
pub enum Value<'buf>{
    Single(&'buf str), //like 'a' in example
    Multiple(Vec<&'buf str>), //like 'd' in example //Length isnot known at compile time, so use Vector.
}

impl<'buf> QueryString<'buf>{
    pub fn get(&self, key:&str) -> Option<&Value>{ //if there is no key in hashmap, it returns None
        self.data.get(key)
    }
}

//We can not use FromStr because it is not supporting types with lifetime parameter, so we will use From trait which one is more generic.
//Also we dont use TryFrom, because the below conversion is guaranteed, every string can be converted into QueryString.
//a=1&b=2&c&d=&e===&d=7&d=abc //Query String example
impl<'buf> From<&'buf str> for QueryString<'buf>{
    fn from(s: &'buf str) -> Self{
        let mut data = HashMap::new();
        s.split('&'); //it gives an iterator which shows the parts of the str
        for sub_str in s.split('&'){
            let mut key = sub_str;
            let mut val = ""; 
            if let Some(i) = sub_str.find('='){
                key = &sub_str[..i];
                val = &sub_str[i+1..] 
            }

            data.entry(key)
                .and_modify(|existing: &mut Value| match existing{ //before inserting potential new value, we can modify the existed value. 
                    Value::Single(prev_val) => { *existing = Value::Multiple(vec![prev_val,val]); },
                    Value::Multiple(vec) => vec.push(val),
                })
                .or_insert(Value::Single(val)); //insert key-value if the key doesnt exist before in the hashmap, else: dont do anything.
        }

        QueryString{data}
    }
}