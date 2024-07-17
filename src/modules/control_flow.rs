#![allow(dead_code)]

// use std::{env, os::unix::thread};
use std::{env, thread, time};

pub fn if_else() {
  let args: Vec<String> = env::args().collect();

  // Arg always will have at least one argument, related to rust debug, your arguments will come after index 0;
  println!("{:?}", args);

  if args.len() < 3 {
    println!("Usage: cargo run <input_file> <output_file>");
    println!("E.g.: cargo run input.txt output.txt");
    return;
  }

  let input_file = &args[1];
  let output_file = &args[2];

  // Read input file
  let input_data = std::fs::read_to_string(input_file).expect("Unable to read input file");

  // Process input data
  let processed_data = process_input_data(input_data);

  // Write processed data to output file
  std::fs::write(output_file, processed_data).expect("Unable to write output file");

  println!("Processing completed. Output saved to {}", output_file);
}

fn process_input_data(input_data: String) -> String {
  // Example processing logic
  // input_data.to_uppercase() // using uppercase function;
  // input_data.to_ascii_lowercase() // using lowercase function;

  input_data
    .chars()
    .map(|c: char| {
      if c.is_uppercase() {
        c.to_ascii_lowercase()
      } else {
        c
      }
    })
    .collect()
}

pub fn infinity_looping() {
  let time_millis = time::Duration::from_millis(300);
  loop {
    let now = time::Instant::now();
    thread::sleep(time_millis);
    println!("\n");
    println!("This loop will never end!");
    println!("Until you stop it: C^-c");
    println!("Time taken: {:?}", now.elapsed());
  }
}

pub fn counter_looping() {
  let mut counter = 0;
  let time_millis = time::Duration::from_millis(1000);

  // loop {
  //   counter += 1;
  //   println!("Counter: {}", counter);
  //   thread::sleep(time_millis);
  // }

  let result = loop {
    counter += 1;
    println!("Counter: {}", counter);
    if counter == 10 {
      break counter;
    }
    thread::sleep(time_millis);
  };
  println!("Loop exited with value: {result}");
}

pub fn counter_nested_looping() {
  let mut count = 0;
  let time_millis = time::Duration::from_millis(1000);
  'counting_up: loop {
    println!("count = {count}");
    let mut remaining = 10;

    loop {
      println!("remaining = {remaining}");
      if remaining == 9 {
        break;
      }
      if count == 2 {
        break 'counting_up;
      }
      remaining -= 1;
    }

    count += 1;
    thread::sleep(time_millis);
  }
  println!("End count = {count}");
}

pub fn while_test() {
  let mut count = 3;

  while count != 0 {
    println!("Count = {}", count);
    count -= 1;
  }
}

pub fn for_in() {
  let numbers = vec![1, 2, 3, 4, 5];

  for number in numbers.iter() {
    println!("Number: {}", number);
  }

  println!();

  for number in (1..4).rev() {
    println!("Number: {}", number);
  }
}
