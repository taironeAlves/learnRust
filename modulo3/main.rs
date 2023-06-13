//declare um objeto carro com 4 campos
struct Car{
    color: String,
    transmission: Transmission,
    convertible: bool,
    mileage: u32
}

#[derive(PartialEq, Debug)]
enum Transmission{
//declare enum do obj Car para o tipo de transmissão
Manual,
SemiAuto, 
Automatic
   }

// Instancie o objeto car para usar valores padrão dos argumentos
// - cor do carro
// - tipo de transmissão enum value
// - Conversivel? true para sim

fn car_factory(color:String, transmission:Transmission, convertible:bool) -> Car {
    //Usando os valores dos argumentos para criar o objeto.
    let mileage = 0u32;
   Car {color, transmission, convertible, mileage}
}


fn main(){
    // Vamos criar 3 carros
    // Vamos criar uma variável mutável para criar os carros
    let mut car = car_factory(String::from("Red"), Transmission::Manual, false);
   println!("Car 1 = {}, Transmission: {:?}, convertible: {}, mileage: {}", car.color, car.transmission, car.convertible, car.mileage);

  car = car_factory(String::from("Silver"), Transmission::Automatic, true);
   println!("Car 2 = {}, Transmission: {:?}, convertible: {}, mileage: {}", car.color, car.transmission, car.convertible, car.mileage);

  car = car_factory(String::from("Yellow"), Transmission::SemiAuto, false);
   println!("Car 3 = {}, Transmission: {:?}, convertible: {}, mileage: {}", car.color, car.transmission, car.convertible, car.mileage);

}