use super::http::{Request,Response,StatusCode,Method};
use super::server::Handler;
use std::fmt::format;
use std::fs;//for file operations

pub struct WebsiteHandler{
    public_path:String, 
}

impl WebsiteHandler{
    pub fn new(public_path:String) -> Self{
        Self{public_path}
    }

    fn read_file(&self, file_path:&str) -> Option<String >{
        let path = format!("{}/{}",self.public_path,file_path);

        match fs::canonicalize(path){ //To prevent/check Directory Traversal Attack
            Ok(path) =>{
                if path.starts_with(&self.public_path){
                    fs::read_to_string(path).ok() //read_to_string copys all file content into string //ok is used for conversion of Result to Option
                }
                else{
                    println!("Directory Traversal Attack Attempted: {}",file_path);
                    None
                }
            }
            Err(_) => None,
        }
        
    }
}

impl Handler for WebsiteHandler{
    fn handle_request(&mut self, request: &Request) -> Response {
        match  request.method(){
            Method::GET => match request.path(){
                //"/" => Response::new(StatusCode::Ok, Some("<h1>Welcome</h1>".to_string())),
                "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
                "/hello" => Response::new(StatusCode::Ok, self.read_file("hello.html")),
                path => match self.read_file(path){ //for example for style.css, linker in index.html searches the paths 
                   Some(contents) => Response::new(StatusCode::Ok,Some(contents)),
                   None => Response::new(StatusCode::NotFound,None),     
                }
                _ => Response::new(StatusCode::NotFound,None),
            },
            _ => Response::new(StatusCode::NotFound,None),
        }   
    }
}

/* 
Directory Traversal Vulnerability: hacker can read any files if the server have a path which is the same name with hacker searchs.
Also using "/.."s, he can goes to root of our server.
It is a vulnerability if we use "path => match self.read_file(path){" statements as above. To prevent this, we must limit that
the files which clients can reach, should be in "public" folder. So the path should be start with "public/". Use "fs::canonicalize" and "starts_with" as above.

*/