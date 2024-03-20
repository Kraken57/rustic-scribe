fn main() {

    let _x: f32 = 3.0; //f32
    let _y = 3.0; //f64


    //boolean
    let _t = true;
    let _s:bool = false;

    //char
    let _c = 'z';
    let _z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("{heart_eyed_cat}");


    //tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let one = tup.1;
    println!("{one}");


    //array
    let a = [1,2,3,4,5];
    let a1: [i32; 5] = [1,2,3,4,5];
    let a2 = [3; 5]; //same as [3,3,3,3,3]
    let first = a[0];
}