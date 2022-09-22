
#[derive(Debug,PartialEq)]
struct EvenNumber(i32);


// impl TryFrom<i32> for EvenNumber {
//     type Error = ();

//     fn try_from(value: i32) -> Result<Self, Self::Error> {
//         if value % 2 == 0 {
//             Ok(EvenNumber(value))
//         }
//         else{
//             Err(())
//         }
//     }
// }

impl TryFrom<i32> for EvenNumber {
    type Error = i32;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        }else {
            Err(1000)
        }
    }
}
fn main(){
    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err((1000)));

    let result: Result<EvenNumber, i32> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    let result: Result<EvenNumber, i32> = 5i32.try_into();
    assert_eq!(result, Err(1000));
 
}