fn main() {
    let divident: i32 = -97;
    let divisor: i32 = 100;
    println!("euclidean quotient: {} / {} = {}",
        divident, divisor, divident.div_euclid(divisor));
    println!("euclidean reminder: {} / {} = {}",
        divident, divisor, divident.rem_euclid(divisor));
}
