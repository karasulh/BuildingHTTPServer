use super::method::Method;
pub struct Request{
    path: String,
    query_string: Option<String>, //Use Option for the case query_string is empty, Option makes it safe. If there is no String, it returns None.
    method: Method, //super::method::Method //Use "super" to reach high level module, like http in this case.
}
