// use  simp::{create_vehicle}  ;
pub mod simp;


use simp::create_vehicle;
fn main() {
    // println!("Hello, world!");
    let vh1 = create_vehicle();
    println!(" Model Name :-  {} \n Year :-  {} \n Color :- {:?} \n Availablity :-  {}", vh1.model, vh1.year, vh1.color, vh1.status);

}


