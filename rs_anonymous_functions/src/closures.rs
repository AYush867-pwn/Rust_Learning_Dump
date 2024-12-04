pub fn test_function(){
    // let add = |x:i8,y:f32| println!("Returning some text with number {:?},{}",x,y);
    let add = |x:i32,y:i32| println!("{}",x+y) ;
    add(-9,34);

    let name  = |m:&str,n:&str| format!("{} {}",m,n);
    let full_name= name("Ayush","Joshi");



    let print_name = || println!("The Name is {}",full_name);
    print_name();


    struct Person{
        
         first_name: String,
         last_name: String,

    }
    
    let mut p1 = Person{first_name: "Ayush".to_string(), last_name:"Kumar".to_string()};

    let mut change_name_closure = |j:&str| p1.last_name = j.to_string();
    change_name_closure("Joshi");
    println!("{}",p1.last_name);






}   


//Closure being anonymous fn can be used to decalare a variable and call a function  , || is a closure