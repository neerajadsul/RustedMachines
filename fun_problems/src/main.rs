
fn main() {

    let args: Vec<String> = std::env::args().collect();
    let number: usize;
    if args.len() == 2 {
        number = args[1].parse().unwrap();
    } else {
        number = 1000;
    }

    // for arg in args {
    //     println!("{}", arg);
    // }
    println!("{}", imperative_sum_of_odd_squares(number as u64));
    println!("{}", functional_sum_of_odd_squares(number as u128));
}

fn is_odd(num: u128) -> bool {
    num % 2 == 1
}

fn imperative_sum_of_odd_squares(upper: u64) -> u128 {
    let mut square:u128 = 1;
    let mut n: u64 = 1;
    let mut total: u128 = 0;
    while square < upper as u128 {
        if is_odd(square) {
            total += square;
        }
        n += 1;
        square = (n * n) as u128;
    }

    total
}

fn functional_sum_of_odd_squares(num: u128) -> u128 {
    let total = 
        (1..num).map(|n| n * n)
                .take_while(|&n_sq| n_sq < num)
                .filter(|&square| is_odd(square))
                .sum();
    total
}
