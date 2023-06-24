use std::io;
use std::num::{ParseFloatError, ParseIntError};

fn read() -> Result<(Result<f64, ParseFloatError>, Result<i64, ParseIntError>), ()> {
    let mut val = String::new();
    match io::stdin().read_line(&mut val) {
        Ok(_) => {
            let val = val.trim();
            Ok((val.parse(), val.parse()))
        }
        Err(_) => Err(()),
    }
}

fn far2cel(f: f64) -> f64 {
    (f - 32.) * 5. / 9.
}

fn cel2far(f: f64) -> f64 {
    1.8 * f + 32.
}

fn fib(n: i64) -> i64 {
    if n < 2 {
        n
    } else {
        let mut a = 0;
        let mut b = 1;
        for _ in 2..=n {
            (a, b) = (b, a + b)
        }
        b
    }
}

fn main() {
    println!("enter value to convert between *F and *C; and to get #th fibonacci number if int value was entered");
    let maybe_val = read();
    match maybe_val {
        Ok(vals) => {
            let (f, i) = vals;
            match f {
                Ok(val) => {
                    println!("{val} F = {} C", far2cel(val));
                    println!("{val} C = {} F", cel2far(val));
                }
                Err(_) => println!("could not parse float"),
            };
            match i {
                Ok(val) => println!("fib({val}) = {}", fib(val)),
                Err(_) => println!("could not parse int"),
            };
        }
        Err(_) => {
            println!("failed to read line");
        }
    }
}
