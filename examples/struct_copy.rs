#[derive(Debug)]
struct Point(i32,i32);

fn main(){
    let p = Point(1,2);
    let p2 = p;
    println!("{:?}",p2);


}

