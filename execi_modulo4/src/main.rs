fn main() {
    let colors = ["Blue", "Green", "Red", "Silver"];

    let mut car: Car;
    let mut engine = Transmission::Manual;

    //Criando um carro
    car = car_factory(String::from(colors[0]), engine, true, 0);
    println!(
        "Car order 1: {:?}, Hard top = {}, {:?}, {}, {} miles",
        car.age.0, car.roof, car.motor, car.color, car.age.1
    );

    engine = Transmission::SemiAuto;
    car = car_factory(String::from(colors[0]), engine, true, 100);
    println!(
        "Car order 2: {:?}, Hard top = {}, {:?}, {}, {} miles",
        car.age.0, car.roof, car.motor, car.color, car.age.1
    );

    engine = Transmission::Automatic;
    car = car_factory(String::from(colors[0]), engine, true, 1);
    println!(
        "Car order 3: {:?}, Hard top = {}, {:?}, {}, {} miles",
        car.age.0, car.roof, car.motor, car.color, car.age.1
    );
}

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
    let quality: (Age, u32);
    if miles > 0 {
        quality = (Age::Used, mileage);
    } else {
        quality = (Age::New, mileage);
    }
    return quality;
}

fn car_factory(color: String, motor: Transmission, roof: bool, miles: u32) -> Car {
    return Car {
        color,
        motor,
        roof,
        age: car_quality(miles),
    };
}
