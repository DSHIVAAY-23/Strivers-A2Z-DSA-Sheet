//  trait Trait {
//     fn f(&self);
// }

// impl<'a> dyn Trait + 'a {
//     fn f(&self) {
//         print!("1");
//     }
// }

// impl Trait for bool {
//     fn f(&self) {
//         print!("2");
//     }
// }

// fn main() {
//     Trait::f(&true);  
//     Trait::f(&true as &dyn Trait); 
//     <_ as Trait>::f(&true);
//     <_ as Trait>::f(&true as &dyn Trait);
//     <bool as Trait>::f(&true);
//     <dyn Trait as Trait>::f(&true as &dyn Trait);
// }
struct S {
    x: i32,
}

const S: S = S { x: 2 };

fn main() {
    let v = &mut S; 
    v.x += 1; // 3 
    S.x += 1; // 4 
    print!("{}{}", v.x, S.x); // 44
 }
//  macro_rules! m {
//     ($($s:stmt)*) => {
//         $(
//             { stringify!($s); 1 }
//         )<<*
//     };
// }

// fn main() {
//     print!(
//         "{}{}{}",
//         m! { return || true }, // 1 << 0
//         m! { (return) || true }, // 
//         m! { {return} || true },
//     );
// }