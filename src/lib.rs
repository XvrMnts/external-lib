#[allow(warnings)]
mod bindings;

use crate::bindings::exports::component::external_lib::duplicate::Guest;

struct Component;

impl Guest for Component {

    fn dupstring(name: String) -> String {
        let res = format!("{} {} {}!!!",name,name,name);
        res.to_string() 
    }
}

bindings::export!(Component with_types_in bindings);
