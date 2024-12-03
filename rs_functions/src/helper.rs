


// Default Main.rs doesnot have accessiblity to read the private functions decalared in the `pub mod helper` so make the function that is been called public by adding `pub` keyword 

pub mod namehelpers{
    //Child Modules of Helpers
    pub fn get_full_name(first:&str,last:&str) -> String{
        let full_name:String= format!("{0} {1}",first,last);
        return  full_name ;
    
    }
}

