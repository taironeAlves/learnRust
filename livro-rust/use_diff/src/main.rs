// A palavra reservada use serve como atalho para os módulos da biblioteca.

/* pub mod a {
    pub mod series{
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

fn main() {

   a::series::of::nested_modules;
}
 */

// Usando o use para abreviar a chamada para módulo.

/* pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

use a::series::of;

fn main() {
    of::nested_modules();
}
 */

// Usando use para abrevia a chamada para a função do módulo

/* pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

use a::series::of::nested_modules;

fn main() {
    nested_modules();
}
 */

// Os enums também são namespaces e podem ser chamados pelo use.

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

use TrafficLight::{Red, Yellow};

fn main() {
    let red = Red;
    let yellow = Yellow;
    let green = TrafficLight::Green;
}
