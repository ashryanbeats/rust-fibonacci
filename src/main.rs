use std::io;

fn main() {
    let n: u32 = get_input();

    let fib_at_n = calc_fib(n);

    println!("The Fibonacci number at index {} is {}.", n, fib_at_n);
}

fn get_input() -> u32 {
  loop {
    println!("Enter the index of the Fibonacci number would you like (0-index).");
    let mut input = String::new();
    
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    match input.trim().parse() {
      Ok(num) => {
        break num
      },
      Err(_) => continue,
    };
  }
}

fn calc_fib(n: u32) -> u32 {
  if n == 0 {
    0
  } else if n == 1 {
    1
  } else {
    let mut o = 2;
    let mut fib = (0 , 1); 

    while o <= n {
      fib = add(fib);
      o += 1;
    }
    fib.1
  }
}

fn add(tup: (u32, u32)) -> (u32, u32) {
  let last = tup.0 + tup.1;
  (tup.1, last)
}