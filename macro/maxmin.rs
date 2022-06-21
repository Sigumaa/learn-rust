macro_rules! find_min {
    ($x:expr) => ($x);
    ($x:expr, $($y:expr),+) => (
        std::cmp::min($x, find_min!($($y),+))
    )
}

macro_rules! find_max {
    ($x:expr) => ($x);
    ($x: expr, $($y:expr),+) => (
        std::cmp::max($x, find_max!($($y),+))
    )
}

fn main() {
    println!("{}", find_min!(1));
    println!("{}", find_min!(1 + 2, 2));
    println!("{}", find_min!(5, 2 * 3, 4));
    
    println!("{}", find_max!(1));
    println!("{}", find_max!(1 + 2, 2));
    println!("{}", find_max!(5, 2 * 3, 4));
}
