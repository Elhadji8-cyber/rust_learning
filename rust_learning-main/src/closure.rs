
struct Person{
  first_name: String,
  last_name: String,
}

pub fn test_closures(){
   
  let add = |x:i32, y:i32|  {
    println!("returning some text {} {}", x, y);
    x + y
  };
  let addition = |x,y| x+ y;
  let result = addition(3, 5);

  let resultart = add(3,5);

  let print_result = |x:i32| println!("the result is: {}", (resultart + x));

  print_result(93);

  let mut p1 = Person{first_name: "Elhadji".to_string(), last_name: "Demba".to_string()};

  let mut change_name = |new_last_name: &str| p1.last_name = new_last_name.to_string();
  let mut change_first_name = |new_first_name: &str| p1.first_name = new_first_name.to_string();
  change_first_name("Kojjo");
  change_first_name("Leuz");
 change_name("Amy");
 println!("{}", p1.last_name);
 println!("{}", p1.first_name);

 
  
  println!("{:}", result)

}