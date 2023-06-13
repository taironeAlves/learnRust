//#[derive(PartialEq, Debug)]
#[derive(PartialEq, Debug)]
struct car {
    color: String,
    motor: Transmission,
    roof: bool,
  
}

enum age {
    mileage(u32),
    Age(u32),
}

enum Transmission{
    Manual,
    SemiAuto,
    Automatic
}

fn main() {}
