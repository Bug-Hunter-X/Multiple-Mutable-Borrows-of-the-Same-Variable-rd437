fn main() {
    let mut x = 5;
    { //Scope the mutable reference for y
        let y = &mut x; 
        *y = 6; 
    }
    { //Scope the mutable reference for z
        let z = &mut x; 
        *z = 7; 
    }
    println!("x = {}", x);
}