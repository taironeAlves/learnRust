use std::collections::HashMap;

fn main() {
    let mut orders = HashMap::new();
    // A tipagem da key é obtida no primeiro insert
    // E a tipagem dos valores em todos os inserts
    let mut order = 1;
    let mut car = car_factory(order, 1000); // retorna Car

    orders.insert(order, car);
    println!("Car order {}: {:?}", order, orders.get(&order));

    // Car order #2: Used, Convertible
    order = order + 1;
    car = car_factory(order, 2000);

    orders.insert(order, car);
    println!("Car order {}: {:?}", order, orders.get(&order));

    // Car order #3: New, Hard top
    order = order + 1;
    car = car_factory(order, 0);
    orders.insert(order, car);
    println!("Car order {}: {:?}", order, orders.get(&order));

    // Car order #4: New, Convertible
    order = order + 1;
    car = car_factory(order, 0);
    orders.insert(order, car);
    println!("Car order {}: {:?}", order, orders.get(&order));

    // Car order #5: Used, Hard top
    order = order + 1;
    car = car_factory(order, 3000);
    orders.insert(order, car);
    println!("Car order {}: {:?}", order, orders.get(&order));

    // Car order #6: Used, Hard top
    order = order + 1;
    car = car_factory(order, 4000);
    orders.insert(order, car);
    println!("Car order {}: {:?}", order, orders.get(&order)); 
}

fn car_factory(order: u32, miles: u32) -> Car {
    let mut colors = ["Blue", "Green", "Red", "Silver"];
    let mut color = order as usize;

    if color > 4 {
        color = color - 4;
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
