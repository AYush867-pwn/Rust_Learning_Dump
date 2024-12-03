

fn main() {
    println!("Hello, world!");  
    // test_if();
    // test_while();      
}
#[allow(dead_code)]
fn test_while(){
    let age_to_drive:u8 = 16;
    let  mut current_age:u8 = 0;
    while  current_age < age_to_drive {
        println!("Waiting....");
        print!("Age was {:?}\n",current_age);
        current_age+=1;

    }

}
#[allow(dead_code)]
fn test_if(){


    let age:u8 = 18 ;

    println!("What's your age ??? ");
    let mut person_age = &mut String::from("");
    std::io::stdin().read_line( person_age).unwrap();


    // Person Age is taken as String that need to be parse to u8 and store it in new var
    let p_age = person_age.replace("\n","").parse::<u8>().unwrap();
    


    if p_age > age{
        println!("You are Allowed to drive !!");
        let driver_license:bool = if p_age > 18{true}else{false};
        println!("{:?}",driver_license);
        
    }
    else if p_age == age {
        println!("You can drive next year Gear up !!");
    }
    else {
        println!("Not Old enough :(");
        println!("Wait for {:?} year more ",age-p_age);
    }


}



fn test_for(){
    let mut ages:[i32;5]=[14,23,25,65,32];
    let age_to_drive:i32= 18;
    for value : i32 in ages{
        println!("The Current Age is {0}",value);
    }
}