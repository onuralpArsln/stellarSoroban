// main func runs just like c++
fn main(){
    
    println!("hello");
    
    // instead of int you have i32 
    let myInt : i32;
    myInt= add(12,12);
    println!(myInt);

    
    // arrays are immutable you nned to make them mutable 
    let arr = [1, 2, 3, 4, 5];
    println!("The first element is: {}", arr[0]);
    // use mut keyword to make them mutable 
    let mut arr = [1, 2, 3, 4, 5];
    arr[0] = 10;
    println!("Updated array: {:?}", arr);



}

// return type ok ile sonda gösterilir pythonda yapabildiğin gibi
fn add (a:i32 , b:i32) -> i32{


        // son hesaplanan değer return value sayılır
        a+b;
}
