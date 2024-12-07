#[allow(dead_code)]
#[derive(Debug)] 
pub enum VehicleColor {
    Red,
    Blue ,
    Black,
    Orange,
    Grey
}
#[allow(dead_code)]
pub struct Vehicle{
    pub model: String,
    pub  year : u16,
    pub color :  VehicleColor,
    pub  status: bool,
}

impl Vehicle{
    fn paint(&mut self, new_color:VehicleColor){
        self.color = new_color;
    }
}

pub fn create_vehicle() -> Vehicle {
    let mut  v1 = Vehicle{model:"Mustang".to_string(),year:1997,color:VehicleColor::Red,status:true};
    v1.paint(VehicleColor::Black);
    return v1;
}