struct Dog{
    name:String,
    age:u8,
}
trait Animal {
    fn make_sound(&self);
    fn bark(&self);
}
impl Animal for Dog{
    fn bark(&self){
        println!("doggy bark plz :{}",self.name);
        println!("doggy bark plz :{}",self.age); }fn make_sound(&self) {
        println!("Woof! My name is {}", self.name);}}
struct Cat{
    name:String,
    age:u8,
}
impl Animal for Cat{
    fn make_sound(&self) {
        println!("Woof! My name is {}", self.name); }
    fn bark(&self){
        println!("doggy bark plz :{}",self.name);
        println!("doggy bark plz :{}",self.age);
    }
}
struct BankAccount {
    balance: f32,
}

impl BankAccount {
    pub fn new(initial_balance: f32) -> BankAccount {
        BankAccount { balance: initial_balance }
    }
    pub fn deposit(&mut self, amount: f32) {
        self.balance += amount;
    }
}
fn main(){
    let burno = Dog{
        name: String::from("Buddy"),
        age:14,
    };
    let pitty = Cat{
        name: String::from("meow"),
        age: 12,
    };
    burno.bark();
    burno.make_sound();
    pitty.bark();
    pitty.make_sound();
}