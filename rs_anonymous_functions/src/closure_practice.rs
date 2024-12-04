pub fn report(){
    

    struct Record{
        name: String,
        age:u8,
    }

    let total_marks = |x:i32,y:i32| -> i32 {x+y};
    let scored_marks:i32 = total_marks(34,56);

    let pass_conf = ||{
        let score:i32 = 100;
        if scored_marks < score {
            println!("Total Marks is {} less than 100",scored_marks);
        }
        else{
            println!("You Passed !!");
        }
    } ;
    pass_conf();
    let   first_record = Record{name:"Ayush".to_string(),age:18};
    let mut second_record: Record = Record{name:"Joshi".to_string(),age:20};
    println!("Name {} and Age {}",first_record.name,first_record.age);
    println!("Name {} and Age {}",second_record.name,second_record.age);
    
    let mut  change_name = |k:&str,l:u8| {
        second_record.name = k.to_string() ;
        second_record.age=l;
    };
    change_name("Kong",89);
    println!("Name {} and Age {}",second_record.name,second_record.age);
    // println!("{}",change_name);






}