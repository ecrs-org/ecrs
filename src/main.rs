use crate::testfunctions::*;

mod testfunctions;

fn main(){
  println!("{}", ackley(&vec![3 as f64, 5.3, 6.1]));
}