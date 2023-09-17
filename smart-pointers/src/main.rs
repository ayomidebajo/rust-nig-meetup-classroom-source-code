// Box shines for recursive programs in Rust
// To illustrate, we create a linked list

use std::rc::Rc;

#[derive(Debug)]
struct Node {
    data: i32,
    next: Option<Rc<Node>>,
}

#[derive(Debug)]
struct LinkedList {
    head: Node,
}

fn main() {
    /* // Create a box pointer
    let b = Box::new(10);
    let c = Box::new(String::from("Hello, world!"));
    // let d = Box::new(vec![10, 5, 5]);

    println!("b = {}", b);
    println!("c = {}", c); */

    // Use the linked list
    /* let ll = LinkedList {
        head: Node {
            data: 20,
            next: Some(Box::new(Node {
                data: 25,
                next: Some(Box::new(Node {
                    data: 30,
                    next: Some(Box::new(Node {
                        data: 35,
                        next: None,
                    })),
                })),
            })),
        },
    };

    println!("{:#?}", ll); */

    // Reference counter example
    // Linked list with some shared data
    // We use Rc::new() for a new reference counter
    // We use Rc::clone(&T) to create a reference count
    let a = Rc::new(Node {
        data: 5,
        next: Some(Rc::new(Node {
            data: 10,
            next: None,
        })),
    });

    let b = Node {
        data: 3,
        next: Some(Rc::clone(&a)),
    };

    let c = Node {
        data: 4,
        next: Some(Rc::clone(&a)),
    };

    // Checking the count
    println!(
        "counts after b and c are created = {}",
        Rc::strong_count(&a)
    );
    {
        // Increasing count in a scope
        let d = Node {
            data: 20,
            next: Some(Rc::clone(&a)),
        };
        println!("counts after d is created = {}", Rc::strong_count(&a));
    }
    println!("counts d goes out of scope = {}", Rc::strong_count(&a));
}
