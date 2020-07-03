macro_rules! printpi {
    ($precision:expr) => {
        let pi = 3.141592;
        println!("Pi is roughly {:.*}", $precision, pi);
    };
}

fn main() {
    // {} is generally replaced by (stringified) arguments
    println!("The number {} is good", 5);

    // We can use positional argument references
    println!("{0} can be less than {1} or {1} can be less than {0}", "x", "y");
    
    // Named arguments also work
    println!("{player} places the {card} on the {surface}",
             player="Michael",
             card="Ace of Spades",
             surface="Chair");

    // Special formatting can be specified after a `:`. b = binary
    println!("{} of {:b} people know binary, the other half don't", 1, 2);

    // specifying >width will right align. can also do < and ^ for other pads
    println!("{number:>width$}", number=1, width=6);
    println!("{number:>width$}", number=21, width=6);
    println!("{number:>width$}", number=4, width=2);
    // of course, we don't have to call this `width`
    println!("{number:>number$}", number=8);
    // we can also pad with 0s. lets write 3 as an 8-bit binary number
    println!("{number:>0width$}", number=11, width=8);

    // We can specify precision in a few ways
    let pi = 3.141592;
    // These will round or truncate depending on the type of your arg
    println!("{:.5}", pi); // inline (hard coded)
    println!("{0:.1$}", pi, 3); // with a positional (or named) reference
    println!("{:.*}", 4, pi); // using a pair of arguments (size, number)

    // lets use a macro to control the precision of a number in string
    printpi!(1);
    printpi!(3);
    printpi!(6);

}