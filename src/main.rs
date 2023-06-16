fn main() {
    println!("u16max = {1}\ni16max = {0}" , std::i16::MAX, std::u16::MAX);

    let x = 5;
    let y = 10.2;
    let z = x * (y as i32);
    let mut status = true;
    println!("z = {ans}", ans=z);

    status = !status;
    println!("status = {ans}", ans=status);
    println!("Values of all variables are {:?}", (x, y, z, status));
    let immut;
    immut = 10;
    println!("immut = {ans}", ans=immut);

    // Initialize multiple variables in one line
    let (a, b, c) = (1_000_000, 2_000, 300);
    println!("a = {a}, b = {b}, c = {c}", a=a, b=b, c=c);


    }
