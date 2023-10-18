use crate::ast::Statement;

mod tokenizer;
mod parser;
mod ast;

fn main() {
    let source_code = "
let x = 5;
func thisIsAVerySimpleInterpreter(t, r) {
    return 5;
}
println(thisIsAVerySimpleInterpreter(5, 6));
";
    let mut parser = parser::Parser::new();

    println!("{}", parser.produce_ast(source_code).to_string());
}

// use rand::Rng;
//
// struct Cat;
// struct Dog;
// struct Duck;
//
// enum Pet {
//     Cat(Cat),
//     Dog(Dog),
//     Duck(Duck),
// }
//
// trait Noise {
//     fn noise(&self);
// }
//
// impl Noise for Cat {
//     fn noise(&self) {
//         println!("Meow");
//     }
// }
//
// impl Noise for Dog {
//     fn noise(&self) {
//         println!("Woof");
//     }
// }
//
// impl Noise for Duck {
//     fn noise(&self) {
//         println!("Quack");
//     }
// }
//
// fn make_noise(pet: &Pet) {
//     match pet {
//         Pet::Cat(c) => c.noise(),
//         Pet::Dog(d) => d.noise(),
//         Pet::Duck(d) => d.noise(),
//     }
// }
//
// fn main() {
//     let cat = Cat;
//
//     make_noise(&Pet::Cat(cat));
//     // let mut rng = rand::thread_rng();
//     // let random_number: u32 = rng.gen();
//     // println!("{}", random_number);
// }


// use std::sync::Arc;
// use std::thread::sleep;
// use std::time::Duration;
// struct Print {
//     value: i32,
// }
//
// impl std::fmt::Display for Print {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(f, "Print: {}", self.value)
//     }
// }
//
// impl std::ops::Shl<i32> for Print {
//     type Output = Print;
//
//     fn shl(self, rhs: i32) -> Self::Output {
//         println!("{}", rhs);
//         return Print { value: self.value };
//     }
// }
//
// fn main() {
//     let foo: Arc<str> = Arc::from("Hello");
//     println!("{}", foo);
//     // sleep(Duration::from_secs(1));
//     // let print = Print { value: 10 };
//     // print << 10;
// }

// #[derive(Clone)]
// struct Node {
//     value: i32,
//     next: Option<Box<Node>>,
// }
//
// struct LinkedList {
//     head: Option<Box<Node>>,
//     tail: Option<Box<Node>>,
// }
//
// impl LinkedList {
//     fn new() -> Self {
//         return LinkedList { head: None, tail: None };
//     }
//
//     fn push(&mut self, value: i32) {
//         let new_node = Node{
//             value,
//             next: None,
//         };
//
//         if self.head.is_none() {
//             self.head = Some(Box::new(new_node.clone()));
//             self.tail = Some(Box::new(new_node));
//             return;
//         }
//
//         let mut current = self.head.as_mut();
//
//         while let Some(node) = current {
//             if node.next.is_none() {
//                 node.next = Some(Box::new(new_node));
//                 self.tail = node.next.clone();
//                 return;
//             }
//
//             current = node.next.as_mut();
//         }
//     }
//
//     fn pop(&mut self) {
//         if self.head.is_none() {
//             return;
//         }
//
//         if self.head.as_ref().unwrap().next.is_none() {
//             self.head = None;
//             self.tail = None;
//             return;
//         }
//
//         let mut current = self.head.as_mut();
//
//         while let Some(node) = current {
//             if node.next.as_ref().unwrap().next.is_none() {
//                 node.next = None;
//                 self.tail = node.next.clone();
//                 return;
//             }
//
//             current = node.next.as_mut();
//         }
//    }
//
//     fn print(&self) {
//         let mut current = self.head.as_ref();
//
//         while let Some(node) = current {
//             println!("{}", node.value);
//             current = node.next.as_ref();
//         }
//     }
// }
//
// fn main() {
//     loop {
//         print!("Enter a number: ");
//     }
//
//     // let mut list = LinkedList::new();
//
//     // list.push(1);
//     // list.push(2);
//     // list.push(3);
//     // list.push(4);
//     // list.push(5);
//
//     // list.print();
//     // println!("------------------");
//     // list.pop();
//     // list.print();
// }
