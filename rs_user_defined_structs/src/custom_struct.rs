#[allow(dead_code)]
pub  struct Student{
     pub name: String,
    pub age: u16,
    pub status : bool,
    pub score:f32,
    pub  fav_place : Place,
}
#[derive(Debug)]
enum Place{
     Delhi,
     Mumbai,
     Banglore,
     Jaipur
}
#[derive(Debug)]
#[allow(dead_code)]
pub struct RecordTuple(String,u16,bool,f32,Place);


pub fn new_record()-> RecordTuple{
    return RecordTuple("krishna".to_string(),34,false,23.1,Place::Mumbai);
}
pub fn create_record() {
    let my_new_record = new_record();
    // println!("Name_t {0} Age_t {1} Active_t {2} Score_t {3} Fav_place_t {:?} ",new_record().0,new_record().1,new_record().2,new_record().3,new_record().4);
}
pub fn student_function() -> Student {
    // let  s1 = Student{name:"Ayush".to_string() ,
    // age: 20,
    // status:true,
    // score:5.6,
    // };

    let mut s2 : Student = Student { name: ("James").to_string(), age: 21 , status: (true), score: (9.2) , fav_place : Place::Jaipur };
    s2.name = "Kornees".to_string()         ;

    // `return s1;
    return s2  ;
}