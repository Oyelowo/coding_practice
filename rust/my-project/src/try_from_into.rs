use std::convert::TryInto;
use std::convert::TryFrom;
use std::convert::From;

#[derive(Debug, PartialEq)]
struct Number {
    value: i32,
}

impl TryFrom<i32> for Number {
    type Error = ();
    fn try_from(item: i32) -> Result<Self, Self::Error> {
        if item > 4 {
        Ok(Number { value: item })
        }else {
            Err(())
        }
    }
}

fn main() {
    assert_eq!(Number::try_from(5), Ok(Number {value: 5}));

    assert_eq!(Number::try_from(3), Err(()));

    let int = 5;
    // Try removing the type declaration
    let num: Result<Number, ()> = int.try_into();
    println!("My number is {:?}", num);
    assert_eq!(num, Ok(Number {value: 5}));
    
    let num: Result<Number, ()> = 3i32.try_into();
    println!("My number is {:?}", num);
    assert_eq!(num, Err(()));
}
