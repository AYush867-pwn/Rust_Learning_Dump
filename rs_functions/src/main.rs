pub mod helper;

 //Child Module of helper module
fn main() {
    println!("Hello, world!");
    let myresult:String = helper::namehelpers::get_full_name("Ayush","Joshi");
    println!("---Code Written by {:?}-----",myresult);
    
}



#[allow(dead_code)]
fn greet(){
    let msg:&str= "Dead Code Allow attribute";
    println!("{:?}",msg);
}