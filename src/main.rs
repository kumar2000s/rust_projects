fn main() {
  let arr = [1,2,3,4,5];
  for i in &arr {
     println!("{} and the length is {}", i, arr.len());
  }

  let tups = (1, "sunil");
     println!("{:?} and the first element is {}", tups, tups.0);

  let st: &str = "dummy string";
     println!("this is {}", st);
  let st1: String = String::from("second string");
     println!("{}", st1);
  //let slice = &st1[1..5];
  //   println!("{}",slice);
  let st2: String = String::from("third string");
     println!("this str1+str2={}", st2 + &st1);



}
