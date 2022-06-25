pub use request::Request;//thanks to this, we can reach Request in a short form in main.rs like "use http::Request;" ,not "use "http::request::Request;"
pub use method::Method;

pub mod request; //we must write this to reach these modules.
mod method; //if we delete "pub",we cant write "use http::method::Method" in main.rs file, because main.rs cannot import all method.rs due to privateness. 
//But thanks to "pub use method::Method", Method contents can be used in main.rs with "use http::Method".