//use std::io;
//const ARR_LEN: usize = 5;


fn main() {
    /*
    let mut x: i32 = 5;
    let y: u32 = 6;
    
    println!("Hello, world! {x} {y}");
    x = 7;
    println!("mut {x}");
    let arr: [i32; ARR_LEN] = [33, 34, 35, 36, 37];
    x = arr[4];
    println!("arr {x}");

    println!("Enter index:");

    let mut index: String = String::new();

    io::stdin() 
        .read_line(&mut index)
        .expect("failed to rl");

    let index: usize = index
        .trim()
        .parse()
        .expect("not a number");

    let element: i32 = arr[index];

    println!{"Value of {index} is: {element}"};
*/
    test_fn(8);
    multiparam_test(43, 's');
    let x: i32 = shadowed_assign();
    println!("return value {x}");

}

fn test_fn(x:i32){
    println!("test_fn call {x}");
}

fn multiparam_test(x:i32, c:char){
    println!("labeled {x}{c}");
}

fn maximum_i32(x:i32, y:i32) -> i32 {
  if x>y {
      return x;
  } 
  else {
      return y;
  }
  
}

fn shadowed_assign() -> i32 {
    let y: f64 = {
        let x: f64 = 3.9;
        x + 1.7
    };
    
    println!("let kekeke {y}");
    //let x: i32 = y.trunc() as i32;
    
    //y.trunc() as i32
    //let y = ;
    println!("kekeke2 {y}");
    return  y.round() as i32 + maximum_i32(5,2);
    //y as i32
}