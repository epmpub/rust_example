fn main(){
    let a = [1,2,3,4,8];
    let s = &a;
    let s1 = &s[0..5];
    println!("{:?}",s1);


    println!("{}",a.get(5).expect("Arrar index out of range."));


}