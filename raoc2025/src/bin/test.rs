fn main() {
    let divident: i32 = -97;
    let divisor: i32 = 100;
    println!("euclidean quotient: {} / {} = {}",
        divident, divisor, divident.div_euclid(divisor));
    println!("euclidean reminder: {} / {} = {}",
        divident, divisor, divident.rem_euclid(divisor));

    let number:u32 = 1188511890;
    let log = number.ilog10();
    println!("{} ilog10 is {}", number, log);
    println!("{} half is {}", number, ((log+1)/2));
    println!("{} lower is {}", number, number % 10_u32.pow((log+1)/2));
    println!("{} upper is {}", number, number / 10_u32.pow((log+1)/2));

    println!();
    let number:u32 = 188511890;
    let log = number.ilog10();
    println!("{} ilog10 is {}", number, log);
    println!("{} half is {}", number, ((log+1)/2));
    println!("{} lower is {}", number, number % 10_u32.pow((log+1)/2));
    println!("{} upper is {}", number, number / 10_u32.pow((log+1)/2));

    let (line, caustic) = ("12345", 42);
    for c in line.chars() {
        println!("{}", c > '3')
    }
    println!("{}", caustic);
    println!("{}", &line[1..])
}
