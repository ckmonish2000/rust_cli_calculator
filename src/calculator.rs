use std::env;

pub fn calculate(){
  let op_1 = env::args().nth(2).expect("need 2 num");
  let symbol = env::args().nth(3).expect("mention the operation");
  let op_2 = env::args().nth(4).expect("need 2 num");

  
  let op_one:f32 = op_1.parse().expect("cound not parse num 1");
  let op_two:f32 = op_2.parse().expect("cound not parse num 2");

  let val = match symbol.as_str() {
    "+"=> op_one+op_two,
    "-"=> op_one-op_two,
    "/"=> op_one/op_two,
    "%"=> op_one%op_two,
    "X" | "x" | "*"=> op_one*op_two,
      _ => panic!("bro you have not passed a valid arg")
  };

  println!("{} {} {} = {}",op_one,symbol,op_two,val);
}