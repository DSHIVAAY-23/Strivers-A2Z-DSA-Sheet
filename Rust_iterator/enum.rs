enum Option<T>{
    Some(T),
    None,

}
enum Err<T,E>{
    Ok(T),
    Err(E),
}
 let heloo = Option::Some(42);
 let absent_number: Option<i32> = None;

fn main(){

}