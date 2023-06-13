#[derive(PartialEq, Debug)]

struct Car {
    color: String,
    motor: Transmission,
    roof: bool,
    age: (Age, u32),
}

#[derive(PartialEq, Debug)]

enum Age {
    New,
    Used,
}

#[derive(PartialEq, Debug)]

enum Transmission {
    Manual,
    SemiAuto,
    Automatic,
}

fn car_quality(miles: u32) -> (Age, u32) {
    let mileage = miles;
    let quality: (Age, u32) = (Age::New, mileage)
}

fn main() {}
