// use std::io;

// fn main(){
//   println!("Guess the number!");

//   println!("Please input your number");

//   let mut guess = String::new();

//   io::stdin()
//       .read_line(&mut guess)
//       .expect("Failed to read line"); 
      
//       println!("you gueese {guess}")
// }

// fn main(){
//   let x = -5;
//   let y: u32=10000;
//   let z: f32= 3.14;

//   print!("x: {x}, y: {y}, z: {z}");
// }

// fn main(){
//   let is_male = true;
//   let is_female = false;
//   if is_male {
//     println!("you are male");
//   }else if is_female {
//       println!("you are female");
//   }
// }

// fn main(){
//   let greeting: String = String::from("Hello, world!");
//   let char1 = greeting.chars().nth(0).unwrap(); //unwrap here mean im ok with the run time expection happens also in rust string is pain in ass
//   //if i remove unwrap its just Option<char> but with unwrap is just char
//   println!("{}", char1);
// }
// fn main(){
//   for i  in 1..100{ //100 excluded
//     println!("{} ", i);
//   }
// }
// fn main(){
//   let sentence = String::from("My name is Aditya");
//   let first_word = get_first_word(sentence);
//   println!("The first word is: {}", first_word);
// }

// fn get_first_word(s:String)->String{
//   let mut ans = String:: new();
//   for char in s.chars(){
//     if char == ' ' {
//       break;
//     }else{
//       ans.push(char);
//     }
//   } 
//   return ans;
// }
 
fn main(){
  let a = 10;
  let b = 20;
  let sum  = do_sum(a , b);
  println!("Here is your sum {sum}");
}

fn do_sum(a:i32,b:i32) -> i32 {
  return a + b;
}
