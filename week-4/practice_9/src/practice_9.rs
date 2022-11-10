fn main() {
    let A:i32 = 10;
    let b:i32 = 20;

    println!("value of A:{}",A);
    println!("value of B : {} ",B);

    let mut res = A>B ;
    println!("A greater than B: {}",res);

    res  = A<B ;
    println!("A lesser than B");

    res = A>=B ;
    println!("A greater or equal to B: {}",res);

    res = A<=B ;
    println!("A is lesser or equal to B") ;

    res = A==B ;
    println!("A lesser than or equal to B: {}",res) ;

    res = A!=B ;
    println!("A is not equalto B: {}",res);
}
