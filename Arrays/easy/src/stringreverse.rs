fn main(){
    let mut st = "helllo bitch";
    let reversed = reverse_string(st);
    println!("{}",reversed);
}

fn reverse_string(st:&str)-> String {
    st.chars().rev().collect()
}