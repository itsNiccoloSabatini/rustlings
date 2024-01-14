// primitive_types3.rs
//
// Create an array with at least 100 elements in it where the ??? is.
//
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand
// for a hint.



fn main() {
    let mut a : [i32 ; 100]= [0; 100];

    let mut _n : i32 = 0;
    let mut c : i32 = 0;

    for _n in 0..100{
        a[_n] = c;
        c+=1;
    }

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed")
    }
    _n=0;
    for _n in 0..100{
        print!(" {} ", a[_n]);
    }
}
