fn main() {
   println!("{:?}", safe_division(9.0, 3.0));
   println!("{:?}", safe_division(4.0, 0.0));
   println!("{:?}", safe_division(0.0, 2.0));
}

#[derive(Debug)]
struct DivisionByZeroError;

fn safe_division(divided:f64, divisor: f64) -> Result<f64, String> {
    
    if divisor == 0.0 {
        Err(String::from("Deu ruim")) // return
    }else {
        Ok(divided / divisor) // return
    }
}

