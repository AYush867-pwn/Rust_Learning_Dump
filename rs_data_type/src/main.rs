fn main() {
    println!("Hello, world!");
    data_types();
}
fn data_types(){
    let x: i8 = 100;
    let y: u8 = 255;
    println!("{:?}",x);
    println!("{:?}",y);

    let z: f32 = 255.342;
    println!("{:?}",z);

    // Type Coercion 

    let a: f32 = 64.34;
    let b: u8 =  50 ;
    let c:f32 = a - b as f32  ;
    println!("{:?}",c);

    // Boolean 
    let mut am_i_old = true;
    am_i_old = false;
    println!("{}",am_i_old);

    // Strings 
    let my_char :char = 'A';
    println!("{}",my_char);

    let my_str: &str = "Ayush";
    println!("{}",my_str);

    let my_emoji:char = '🔥';
    println!("{}",my_emoji);    

    let mut  first_name = "Ayush";
    let first_name : &str = "kr3pt0";
    println!("{}",first_name);


    // Tuple 
    let name: (&str,&str,i32,bool)  = ("Ayush","Joshi",20 ,true); 
    println!("{:?}",name);

    //Array 
    let fruits: [&str;4] = ["Apple","Jelly","Bannana","Orange"];
    println!("{:?}",fruits);
    
    let numbers:[i8;5] = [1,2,3,4,5];


    //Slices
    let even_numbers = &numbers[1..=4];
    println!("{:?}",even_numbers);
}

