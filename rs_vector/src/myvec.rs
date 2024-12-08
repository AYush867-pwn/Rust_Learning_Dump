#[warn(unused_imports)]
// use core::slice::SlicePattern;
pub  fn test_vec_init(){
    let mut my_ints : Vec <i32> = Vec::new();
    my_ints.push(10);
    my_ints.push(20);
    my_ints.push(30);
    my_ints.push(40);
    my_ints.push(50);
    my_ints.push(60);
    my_ints.push(70);
    my_ints.pop();
    my_ints.push(70);
    my_ints.push(80);
    println!("{:?}",my_ints);
    println!("Length of Vector is :- {}",my_ints.len());
    println!("Capacity of Vector is :- {}",my_ints.capacity());

    // println!("First 3 Slice of Vector {:?} ", &(&my_ints).as_slice()[0..=3]);
    println!("First Elements of Vec :- {:?}",my_ints.get(0));
    
}

pub fn  test_vec_string(){
    let names:Vec<&str> = vec!["Jones","Karles","Maya","Leo"];
    for x in names.clone(){
        println!("Processing ...... {} ",x);
    }
    println!("Name Vector : {:?} ",names);

}

#[derive(Debug)]
struct Record {
    Name : String,
    Present : bool,
}
#[derive(Debug)]
struct Marks {
    subject : String,
    score : bool,
}
#[warn(unused_mut)]
pub fn test_vec_custom(){
    let mut vec_rec : Vec<Record> = vec![];
    let mut vec_mark : Vec<Marks> = vec![];
    let mut vec_rec2 : Vec<Record> = vec![];
    for i in 1..=2{
      vec_rec.push(Record{Name:"Jones".to_string(),Present:true}) ;
    }
    for i in 1..=2u8{
        vec_rec2.push(Record{Name:"Karls".to_string(),Present:true}) ;
    }
    vec_rec.append(&mut vec_rec2);
    vec_rec.insert(0,Record { Name: ("Sunny").to_string(), Present: (true) });
    println!("{:?}\n",vec_rec);
    println!("{:?}",vec_rec2);
    println!("\nLength of Record  :- {}\n",vec_rec.len());
    println!("\nCapacity of Record  :- {}",vec_rec.capacity());
    println!("\nLength of Second   :- {}\n",vec_rec2.len());
    println!("\nLength of Second :- {}\n",vec_rec2.capacity());

    println!("{:?}",vec_rec.get(0));
    vec_rec.remove(0);
    println!("{:?}",vec_rec.get(0));
    
    
    /* USING CLOSURES OR ANONYMOUS FUNCTION TO RETURN TRUE OR FALSE TO RETAIN A VECTOR */
    vec_rec.retain(|e:&Record|{if e.Name=="Jones" {return true;} else {return false}});
    println!("{:?}\n",vec_rec);

    /*Additional Capacity Reserve using reserver() adds the unit to the current capacity */

    vec_rec.reserve(900);
    println!("\nLength of Second :- {}\n",vec_rec.capacity());


}   