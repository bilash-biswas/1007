fn main(){
   let mut line1 = String::new();
   std::io::stdin().read_line(&mut line1).unwrap();
   let num1: i32 = line1.trim().parse().ok().expect("Try");
   
   let mut line2 = String::new();
   std::io::stdin().read_line(&mut line2).unwrap();
   let num2: i32 = line2.trim().parse().ok().expect("Try");
   
   let mut line3 = String::new();
   std::io::stdin().read_line(&mut line3).unwrap();
   let num3: i32 = line3.trim().parse().ok().expect("Try");
   
   let mut line4 = String::new();
   std::io::stdin().read_line(&mut line4).unwrap();
   let num4: i32 = line4.trim().parse().ok().expect("Try");

   println!("DIFERENCA = {}", (num1 * num2 - num3 * num4))
}
