fn main() {
    let mut count = 1;
    loop {
        count += 1;
        println!("{}", count);

        if count == 1000000000 {
            println!("{}", count);
            break;
        }
    }
}
