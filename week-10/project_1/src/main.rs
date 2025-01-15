struct Laptop {
    brand:String, price:u32, quantity:u32
}




fn main() {
    println!("Hello! Welcome to Our Store");
    println!("The Following Laptops are available in stock");
    println!("Brand: HP \nPrice: 650,000 \nQty Available: 10\n\n");
     println!("Brand: IBM \nPrice: 755,000 \nQty Available: 6\n\n");
      println!("Brand: Toshiba \nPrice: 550,000 \nQty Available: 10\n\n");
       println!("Brand: Dell \nPrice: 850,000 \nQty Available: 4\n\n");

       let hp = Laptop{
        brand:String::from("HP"),
        price: 650_000, 
        quantity: 10
       };

       let ibm = Laptop{
        brand:String::from("IBM"),
        price: 755_000, 
        quantity: 6
       };

       let toshiba = Laptop{
        brand:String::from("Toshiba"),
        price: 550_000, 
        quantity: 10
       };

       let dell = Laptop{
        brand:String::from("Dell"),
        price: 850_000, 
        quantity: 4
       };

       let total = 3 * (hp.price + ibm.price + toshiba.price + dell.price);

    println!("\nThe customer purchased 3 laptops \nfrom each brand making the total cost: \n\n${}", total);

}


