
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
    println!("{}", imperative_sum_of_odd_squares(number as usize));
    println!("{}", functional_sum_of_odd_squares(number as usize));
}

fn is_odd(num: usize) -> bool {
    num % 2 == 1
}

fn imperative_sum_of_odd_squares(upper: usize) -> usize {
    let (mut square, mut n) = (1, 1);
    let mut total: usize = 0;
    while square < upper {
        if square % 2 == 1 {
            total += square;
        }
        n += 1;
        square = n * n;
    }

    total
}

fn functional_sum_of_odd_squares(num: usize) -> usize {
    let total = 
        (1..num).map(|n| n * n)
                .take_while(|&n_sq| n_sq < num)
                .filter(|&square| is_odd(square))
                .sum();
    total
}
