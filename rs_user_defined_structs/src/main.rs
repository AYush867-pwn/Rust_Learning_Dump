use custom_struct::{create_record, student_function};

pub mod custom_struct;


fn main() {
    // println!("Hello, world!");
    let student_1 = custom_struct::student_function();
    // println!("{} {} {} {} {:?} ",student_1.name,student_1.age,student_1.score,student_1.status ,student_1.fav_place);
    println!("Name {} Age {} Status {} Score {}",student_function().name,student_function().age,student_function().status,student_function().score);
    // create_record();
    // student_function();     
}

