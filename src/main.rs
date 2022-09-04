  // w: 119, a: 97, s: 115, d: 100
extern crate termios;
use std::io;
use std::io::Read;
use std::io::Write;
use std::cmp::Ordering;
use termios::{Termios, TCSANOW, ECHO, ICANON, tcsetattr};
use colored::Colorize;
use rand::Rng;
  fn main()
  {
println!("{}", format!("
  ______ _     _ _______ _______ _______
 |  ____ |     | |______ |______ |______
 |_____| |_____| |______ ______| _______|").red().bold());
println!("{} {}", format!("
            ____ _  _ ____   
              |  |__| |___   
              |  |  | |___").green().bold(),
 format!("
        
        _  _ _  _ _  ____ ___ ____
        |\\ | |  | |\\/|__]|___ |__/ 
        | \\| |__| | ||__]|___ |  \\ ™").blue().bold());
 println!("\n© Ethan Lee, 2022\n\n\n");
print!("Choose a difficulty\n(Press W or S to move, Press A to confirm.)\n\n\r");
   let diff:[&str; 5] = ["NaN",">Easy | Normal | Hard | Custom", " Easy |>Normal | Hard | Custom", " Easy | Normal |>Hard | Custom", " Easy | Normal | Hard |>Custom"];
   let mut index: usize = 1;
   
  loop
  {

    let stdin = 0;
    let termios = Termios::from_fd(stdin).unwrap();
    let mut new_termios = termios.clone();
    new_termios.c_lflag &= !(ICANON | ECHO);
    tcsetattr(stdin, TCSANOW, &mut new_termios).unwrap();
    let  stdout = io::stdout();
    let mut reader = io::stdin();
    let mut buffer = [0;1];
    stdout.lock().flush().unwrap();
    reader.read_exact(&mut buffer).unwrap();
    tcsetattr(stdin, TCSANOW, & termios).unwrap();
   if buffer == [119] 
    {
    index += 1;
    if index > 4
    {
      index -= 1;
    }
    print!("\r{}", diff[index]);
    }
   else if buffer == [115]
   {
    index -= 1;
    if index == 0
    {
      index += 1;
    }
    print!("\r{}", diff[index]);
   }


  else if buffer == [97]
  {
   start(index);
   break;
  }
}
     }
fn start(mut difu: usize)
{
let mut attemps: u64 = 0;
if difu == 1
{
  attemps = 10;
  difu = 25;
  println!("\n\nGuess a number between 1 to {}, You have {} attemps. Good Luck!", difu, attemps);
}
else if difu == 2
{
  attemps = 8;
  difu = 40;
println!("\n\nGuess a number between 1 to {}, You have {} attemps. Good Luck!", difu, attemps);
}
else if difu == 3
{
  attemps = 4;
  difu = 70;
  println!("\n\nGuess a number between 1 to {}, You have {} attemps. Good Luck!", difu, attemps);
}
else if difu == 4
{
  let mut ix = String::new();
  println!("\n\nInput Maximum Number Range:");
  io::stdin().read_line(&mut ix);

 difu = ix.trim().parse().expect("Input not an integer");
 let mut iz = String::new();
println!("Input how many Attemps:");
  io::stdin().read_line(&mut iz);
attemps = iz.trim().parse().expect("Input not an integer");
  println!("\n\nGuess a number between 1 to {}, You have {} attemps. Good Luck!", difu, attemps);
}
let secret_number = rand::thread_rng().gen_range(1..=difu);
loop {
println!("\n{}", format!("Please input your guess:").purple());
let mut guess = String::new();
io::stdin()
.read_line(&mut guess)
.expect("Failed to read line");
let guess: usize = match guess.trim().parse() {
Ok(num) => num,
Err(_) => continue,
};
println!("{} {guess}", format!("You guessed:").cyan());
if attemps == 0
{
 println!("You Lose. Better luck next time!");
 break;
}
match guess.cmp(&secret_number) {
Ordering::Less => {
println!("{}", format!("Too small!").yellow()); attemps -= 1;
},
Ordering::Greater => {
attemps -= 1;
println!("{}", format!("Too big!").red()); 
},
Ordering::Equal => {
println!("{}", format!("🦀 You win! 🦀").green());
break;
}
  }
   }
}
