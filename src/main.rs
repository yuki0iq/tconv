use std::io;

fn read() -> f64 {
    let mut val = String::new();
    io::stdin().read_line(&mut val).expect("enter please");
    val.trim().parse().expect("number please")
}

fn far2cel(f: f64) -> f64 {
    (f - 32.) * 5. / 9.
}

fn cel2far(f: f64) -> f64 {
    1.8 * f + 32.
}

fn main() {
    println!("enter value to convert between *F and *C");
    let val = read();
    println!("{val} F = {} C", far2cel(val));
    println!("{val} C = {} F", cel2far(val));
}
