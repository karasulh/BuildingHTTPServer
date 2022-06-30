pub use request::Request;//thanks to this, we can reach Request in a short form in main.rs like "use http::Request;" ,not "use "http::request::Request;"
pub use method::Method;
pub use request::ParseError;
pub use query_string::{QueryString,Value as QueryStringValue}; //Value is generic, so we changed its name which is use to reach it.
pub use response::Response;

pub mod response;
pub mod query_string;//to define queryString for Request
pub mod request; //we must write this to reach these modules.
pub mod method; //if we delete "pub",we cant write "use http::method::Method" in main.rs file, because main.rs cannot import all method.rs due to privateness. 
//But thanks to "pub use method::Method", Method contents can be used in main.rs with "use http::Method".
