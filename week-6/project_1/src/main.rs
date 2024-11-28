use std::io;

fn main() {
    println!("Welcome To Our Restaurant! ");
    println!("The following items on our menu are available:");
    
    println!("Poundo Yam/Edinkaiko Soup (p) ------> N3,200
         Fried Rice & Chicken (f) ------> N3,000
         Amala & Ewedu Soup (a) ------> N2,500
         Eba & Egusi Soup (e) -----> N2,000
         White Rice & Stew (w) ------> N2,500");

    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("What do you want to order? \nUse the letter codes (p, f, a, e, w) ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let order = input1.trim().to_uppercase();

    println!("Quantity of meal?");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let qty:i32 = input2.trim().parse().expect("Not a valid number");

    let p:i32 = 3200;
    let f:i32 = 3000;
    let a:i32 = 2500;
    let e:i32 = 2000;
    let w:i32 = 2500;

    

    if order == "P" {
        let price = p*qty;
        println!("Price is N{}",price);
        if price > 10000 {
        println!("A discount of 5% has been applied");
        let new_price = 0.95*price as f32;
        println!("Your new total is N{}",new_price); 
    } 
}
    else if order == "F" {
        let price = f*qty;
        println!("Price is N{}",price);
        if price > 10000 {
        println!("A discount of 5% has been applied");
        let new_price = 0.95*price as f32;
        println!("Your new total is N{}",new_price); 
    } 
}
    else if order == "A" {
        let price = a*qty;
        println!("Price is N{}",price);
        if price > 10000 {
        println!("A discount of 5% has been applied");
        let new_price = 0.95*price as f32;
        println!("Your new total is N{}",new_price); 
    } 
}
    else if order == "E" {
        let price = e*qty;
        println!("Price is N{}",price);
        if price > 10000 {
        println!("A discount of 5% has been applied");
        let new_price = 0.95*price as f32;
        println!("Your new total is N{}",new_price); 
    } 
}
    else if order == "W" {
        let price = w*qty;
        println!("Price is N{}",price);
        if price > 10000 {
        println!("A discount of 5% has been applied");
        let new_price = 0.95*price as f32;
        println!("Your new total is N{}",new_price); 
    }
}

    

}

