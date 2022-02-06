use std::env;
mod calculator;
mod help;

fn main() {

  let flag = env::args().nth(1).expect("please pass a flag");

  let check_flag =flag.chars().next().unwrap();
  let flag_start = "-".chars().next().unwrap();
  assert_eq!(check_flag,flag_start);

  if flag.as_str() == "-c"{
    calculator::calculate();
  }else if flag.as_str() == "-h"{
    help::print_help();
  }else{
      panic!("Invalid flag")
  }

}
