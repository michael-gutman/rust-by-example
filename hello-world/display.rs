use std::fmt; // import things with use

// we will manually implement a nice Display (print) for this struct
struct Printable(i32);
// this will let us use {}
impl fmt::Display for Printable {
    // the 'trait' fmt::Display needs this exact signature
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        // self.x refers to the positional argument x
        write!(f, "{}", self.0)
    }
}

// Here's a more custom one, with named arguments
#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

// Exercise: make a complex number struct which prints like: 
// Display: 3.3 + 7.2i
// Debug: Complex { real: 3.3, imag: 7.2 }
#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}


fn main() {
    println!("We can print this struct: {}", Printable(11));

    let point = Point2D{ x: 6.3, y: 3.6 };
    println!("Display: {}", point);
    println!("Debug (derived): {:?}", point);

    let c1 = Complex{ real: 3.3, imag: 7.2 };
    println!("Display: {}", c1);
    println!("Debug (derived): {:?}", c1);
}