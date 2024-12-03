use std::io;

fn trapezium(){
    let mut input2 = String::new();
    println!("Enter a value for base 1");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let base1:f32 = input2.trim().parse().expect("Invalid number");

    let mut input3 = String::new();
    println!("Enter a value for base 2");
    io::stdin().read_line(&mut input3).expect("Failed to read input");
    let base2:f32 = input3.trim().parse().expect("Invalid number");

    let mut input4 = String::new();
    println!("Enter a value for height");
    io::stdin().read_line(&mut input4).expect("Failed to read input");
    let h:f32 = input4.trim().parse().expect("Invalid number");
    
    let area_trap:f32 = 0.5 * (base1 + base2) * h;
    println!("Area of trapezium: {}",area_trap);
}

fn rhombus(){
    let mut input5 = String::new();
    println!("Enter a value for diagonal 1");
    io::stdin().read_line(&mut input5).expect("Failed to read input");
    let diag_1:f32 = input5.trim().parse().expect("Invalid number");

    let mut input6 = String::new();
    println!("Enter a value for diagonal 2");
    io::stdin().read_line(&mut input6).expect("Failed to read input");
    let diag_2:f32 = input6.trim().parse().expect("Invalid number");

    let area_rhom:f32 = 0.5 * diag_1 * diag_2;
    println!("Area of rhombus: {}",area_rhom);
}

fn parallelogram() {
    let mut input7 = String::new();
    println!("Enter a value for the base");
    io::stdin().read_line(&mut input7).expect("Failed to read input");
    let basep:f32 = input7.trim().parse().expect("Invalid number");

    let mut input8 = String::new();
    println!("Enter a value for the altitude");
    io::stdin().read_line(&mut input8).expect("Failed to read input");
    let altitude:f32 = input8.trim().parse().expect("Invalid number");

    let area_parr:f32 = basep * altitude ;
    println!("Area of parallelogram: {}",area_parr);
}

fn cube() {
    let mut input9 = String::new();
    println!("Enter a value for the length of the cube");
    io::stdin().read_line(&mut input9).expect("Failed to read input");
    let length:f32 = input9.trim().parse().expect("Invalid number");

    let area_cube:f32 = 6.0 * length * length;
    println!("Surface Area of cube: {}",area_cube);
}

fn cylinder() {
    let mut input10 = String::new();
    println!("Enter a value for the radius");
    io::stdin().read_line(&mut input10).expect("Failed to read input");
    let radius:f32 = input10.trim().parse().expect("Invalid number");

    let mut input11 = String::new();
    println!("Enter a value for the height");
    io::stdin().read_line(&mut input11).expect("Failed to read input");
    let height:f32 = input11.trim().parse().expect("Invalid number");

    let vol_cylinder:f32 = (22.0 / 7.0) * radius * radius * height;
    println!("Volume of cylinder: {}",vol_cylinder);
}


fn main() {
    println!("Select what you want to calaculate:");
    println!("Area of trapezium (t)
        \nArea of rhombus (r)
        \nArea of parallelogram (p)
        \nSurface Area of cube (ac)
        \nVolume of cylinder (vc)");

let mut input1 = String::new();    

println!("Enter the code of what you want to calculate: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let code = input1.trim().to_lowercase().to_string();

    if code == "t" {
        trapezium();
    }
    else if code == "r"{
        rhombus();
    }
    else if code == "p" {
        parallelogram();
    }
    else if code == "ac" {
        cube();
    }
    else if code == "vc" {
        cylinder();
    }
    else{
        println!("Invalid input");
    }

}
