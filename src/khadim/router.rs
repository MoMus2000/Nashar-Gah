use std::collections::HashMap;
use super::response::{Request, ResponseWriter};

#[derive(Clone)]
pub struct Router{
    pub router_elem_mapper: HashMap<String, RouterElement>
}

impl Router {
    pub fn new() -> Router{
        Router{
            router_elem_mapper: HashMap::new()
        }
    }

    pub fn add_route(&mut self, path: &'static str, method: &'static str, callback_function: fn(Request, ResponseWriter) -> String) -> bool{
        let element = RouterElement{
            path,
            callback_function,
            method
        };

        if self.router_elem_mapper.contains_key(path){
            return false;
        }
        else{
            self.router_elem_mapper.insert(path.to_string(), element);
        }
        true
    }

    pub fn fetch_func(&self, path: &str, method: &str) -> Option<fn(Request, ResponseWriter) -> String>{
        if self.router_elem_mapper.contains_key(path){
            let re = self.router_elem_mapper.get(path).unwrap();
            if re.method == method {
                return Some(re.callback_function)
            }
        }
        None
    }

}

#[derive(Clone, Debug)]
pub struct RouterElement {
    pub path: &'static str,
    pub callback_function: fn(Request, ResponseWriter) -> String,
    pub method: &'static str
}