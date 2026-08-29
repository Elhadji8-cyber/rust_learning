pub mod helpers;
use std::io;


fn main() {
    println!("Hello, world!");

    //test_func();
   //let my_result = helpers ::namehelpers:: get_full_name("Elhadji", "Demba");

   //let new_age =helpers::database::get_age_plu_5(23);


   //print!("Hello from {0}\n", my_result);

    //print!("The new age is:{0}", new_age);
   // test_if();
   test_while();
   test_loop();
   test_for()
}


fn test_for(){


    let ages = [14,18,26,35,41];
    let age_to_driver = 16i32;

    for value in ages{
        //println!("the current age is {0}", value);  
        if value >=age_to_driver{
            println!("you are old enough to drive");
        }else {
            println!("You nedd a license");
        }
    }
}
fn test_loop() {

    //x
    let mut x =1;
   loop {
    println!("Hello from rust");

    if x > 5{
        break;
    }
    x+=1;
   }
}

fn test_while(){
  

  let age_to_drive = 16u8;
  let mut current_age = 0u8;

  while current_age < age_to_drive {
    println!("waiting...");
    current_age += 1;

    if current_age == 6 ||current_age == 5{
        break;
    }
  }

}


#[allow(dead_code)]
fn test_if(){
    let age_to_drive : u8 = 16u8;

    println!("Enter the person's age:");
    let mut myinput: &mut String = &mut String::new();
    io::stdin().read_line(&mut myinput).unwrap();

    let age : u8 = myinput.trim().parse::<u8>().unwrap();
    if age >= age_to_drive {
        println!("Issuing dreiver's license, because they are old enough");
    }else if age == 16 || age > 14{
         println!("you are just on the verge being old enough! wait one more year.")
    }else{
        println!("wait a bit longer, you aren't old enough for a driver's license")
    }

    let drivers_license = if age >= 16 {true} else{false};
    println!("{:?}",drivers_license)

    

}




#[allow(dead_code)]
fn test_func(){

    let entier: i32 = 5;
    let flottant: f64 = 2.5;
    let result: f64 = (entier as f64) + flottant; // Conversion de entier en f64 pour l'addition
    let x: f32 = 255.0;
    let y: u8 = x as u8 - 5;

    let name = ("elhadji", ("Demba"), 40 as u8);
    //arrays
    let ages = [40,45,50, 55, 60, 65 as u8];

    let new_ages = &ages[1..4];

    print!("{:?}", new_ages);

    print!("{:?}", name);
    print!("{:?}", ages);
  

    println!("{:?}", y);
    println!("{:?}", result);
    println!("{:?}", x);
}