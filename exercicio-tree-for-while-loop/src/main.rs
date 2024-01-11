use std::collections::HashMap;

fn main() {
    let mut miles = 0;

    let mut orders: HashMap<u32, Car> = HashMap::new();

    let mut order = 1;
    // for number in 1..8 {

    //     let mut car = car_factory(number, miles);
    //     orders.insert(number, car);
    //     println!("Car order {}: {:?}", number, orders.get(&number));
    // }

    loop {
        let mut car = car_factory(order, miles);
        orders.insert(order, car);
        println!("Car order {}: {:?}", order, orders.get(&order));

        order += 1;

        if miles == 2100 {
            miles = 0;
            // break;
        } else {
            miles += 700;
        }
    }
}

fn car_factory(order: u32, miles: u32) -> Car {
    let mut colors = ["Blue", "Green", "Red", "Silver"];
    let mut color = order as usize;

    while color > 4 {
        color -= 4;
    }

    let mut motor = Transmission::Manual;
    let mut roof = true;

    if order % 3 == 0 {
        motor = Transmission::Automatic;
    } else if order % 2 == 0 {
        motor = Transmission::SemiAuto;
        roof = false;
    }

    Car {
        color: String::from(colors[(color - 1) as usize]),
        motor,
        roof,
        age: car_quality(miles),
    }
}

fn car_quality(miles: u32) -> (Age, u32) {
    // Essa função irá retornar um enumeração Age do tipo inteiro de 32bits

    if miles > 0 {
        return (Age::Used, miles);
    }

    (Age::New, miles)
}

#[derive(PartialEq, Debug)]
// Struct é semelhante a obj, a diferença é que struct armazena apenas as propriedades, enquanto obj armazenam propriedades e metodos.
// As propriedades de um struct são imutáveis por padrão, sendo necessário especificar o mut

struct Car {
    color: String,
    motor: Transmission,
    roof: bool,
    age: (Age, u32),
}

#[derive(PartialEq, Debug)]
enum Transmission {
    Manual,
    SemiAuto,
    Automatic,
}

#[derive(PartialEq, Debug)]
enum Age {
    New,
    Used,
}
