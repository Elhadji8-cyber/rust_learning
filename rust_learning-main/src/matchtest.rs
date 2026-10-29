


pub fn test_match_string(){
  let car_manufacturer= "Porche";

  match  car_manufacturer {
      
      "Hyundai" =>{
        println!("This is Hyundai");
      }
      "Porche" =>{
           println!("This is my car Porche");
      }
      _ =>{
        println!("witch car is this")
      }
  }
}

pub fn test_match_int(){

  let my_age:u16 =60;
  let grown:u16 = 40;
  //literal pattern
  match my_age{
    35 =>{
        println!("your age is 35")
    }
    60 =>{
          println!("your age is 60 you can be somoene grand-father or mother")
    }
    _=>{
        println!("your age is not 35")
    }
  }

  //reference patterns
   match grown{
    16..=20 =>{
       println!("your are young")
    }
    30..=40 =>{
        println!("your are old enough to be someone father or mother")
    }
    50..=70=>{
       println!(" can you be my grand-pa or grand-ma")
    }
    _ =>{
       println!("i don't know your age")
    }
   }

}

pub fn test_macth_array(){
  let prices = [30000_u32,50000, 90000,120000 ];

  match  prices[0..=3]{
      [30000, 50000] =>{
        println!("you have some reasonable priced cars");
      }
      [30000,50000, ..] =>{
           println!("you have some reasonable priced cars tha match my money");
      }
      _ =>{
        println!("you don't have any reasonable priced cars here");
      }
  }

}

































