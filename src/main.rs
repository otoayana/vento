use std::env;
use colored::Colorize;

mod inv;

fn main() {
   let args: Vec<String> = env::args().collect();
   if args.len() >= 2 {
       match args[1].as_str() {
            "init" => inv::init(),
            "list" => {
                if args.len() == 3 {
                    inv::list(args[2].as_str());
                } else {
                    inv::list("active");
                };
            },
            "switch" => inv::switch(),
            _ => println!("❔ Command not found")
       }
   } else {
       println!("{} by nixgoat", format!("Vento").bold().blue());
   }
}
