fn main() {
    let r3 = 10551425;
    let mut r0 = 0;
    for i in 1..=r3 {
        if (r3 / i) * i == r3 {
            println!("factor {}", i);
            r0 += i;
        }
    }
    println!("r0 = {}", r0);
}
