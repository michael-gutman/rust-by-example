// formatting can be done with {} or {:?}
// {:?} is for debugging purposes
// while {} needs to be manually implemented for any non std types
// {:?} can be automatically derived for any type
// but you don't have any control over how this will look
fn main() {
    // This struct cannot be printed in a formatted string
    #[allow(dead_code)]
    struct UnPrintable(i32);
    // (also, all this struct seems to do is take an int and hold it)

    // we can derive debug to be able to use {:?} to print this struct
    #[derive(Debug)]
    struct DebugPrintable(i32);
    // the following lines won't even compile: 
    // println!("Printing this: {} doesn't work!", UnPrintable(5));
    // println!("Printing this: {:?} doesn't work either", UnPrintable(34));
    // println!("this one too {}", DebugPrintable(55));

    println!("This one {:?} does work though", DebugPrintable(853));

    // Note that you won't be able to derive debug in a case like this
    // where your struct contains a struct that hdoesn't implement debug
    // #[derive(Debug)] 
    // struct UnPrintableInside(UnPrintable)
    // ^ this won't compile

    // more complicated structs can at least be 'pretty printed' with {:#?}
    // but we'll get to those later :P
}