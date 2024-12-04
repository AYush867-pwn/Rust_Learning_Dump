pub fn test_match(){

    let myage:u16 = 92;

    match myage{
        1..=20 => {println!("Your Age is between 1 - 20");}
        21..=50 => {println!("You are adult ");}
        51..=70 => {println!("You are Senior citizen");}
        71..=100 => {println!("You are Legend");}
        _ => {println!("Default Value Reached ");}
    }
}

pub fn test_string(){
    let model:&str = "POCO";

    match model{
        "SAMSUNG" => println!("SAMSUNG MODEL "),
        "XIOMI" => println!("MI PHONES"),
        "POCO" =>  println!("BUDGET GAMING PHONES"),
        _ => println!("MODEL NOT DEFINED"),
    };
}

pub fn test_array(){
    let students:[&str;5] = ["Ayush","James","Khalessi","Rock","Angle"];

    match students{
        ["Ayush","James","Khalessi","Rock",..] => println!("Everyone is Present"),
        ["Ayush",..] => println!("Monitor Present Confirmed"),
        _ => println!("Default value reached "),
    };
}