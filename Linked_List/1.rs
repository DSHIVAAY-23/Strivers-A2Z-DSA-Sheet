fn main() {
let mut string1 = String::from("hello world");
let mut string2 = String::from("rust programming");




}

fn longest(x:&str, y:&str) -> &str{
    if x.len() > y.len(){
        x
    } else {
        y
    }
}
fn smallest(x:&String, y:&String) -> &String{
    if x.len() < y.len(){
        x
    } else {
        y
    }
}