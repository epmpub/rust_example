use std::{str::FromStr, fmt::Display};
#[derive(Debug)]
struct Money {value:i32}

impl Display for Money {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"output is {}",self.value)
    }
}

impl FromStr for Money  {
    type Err = i32;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.is_empty() {
            Ok(Money{value:100})
        }else {
            Err(-1)
        }
    }
}
#[allow(unused_variables)]
fn main(){
    let ss = Money{value:33};
    let ss = Money::from_str("10").unwrap();
    println!("{:?}",ss.to_string());
}