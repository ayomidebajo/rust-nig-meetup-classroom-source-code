// Box shines for recursive programs in Rust
// To illustrate, we create a linked list

/* #[derive(Debug)]
struct Node {
    data: i32,
    next: Option<Box<Node>>,
}

#[derive(Debug)]
struct LinkedList {
    head: Node,
} */

fn main() {
    // Create a box pointer
    let b = Box::new(10);
    let c = Box::new(String::from("Hello, world!"));

    println!("b = {}", b);
    println!("c = {}", c);

    /* // Use the linked list
    let ll = LinkedList {
        head: Node {
            data: 20,
            next: Some(Node {
                data: 25,
                next: Some(Node {
                    data: 30,
                    next: Some(Node {
                        data: 35,
                        next: None,
                    }),
                }),
            }),
        },
    }; */

    // println!("{:#?}", ll);

    // Reference counter example
    // Linked list with some shared data
    // We use Rc::new() for a new reference counter
    // We use Rc::clone(&T) to create a reference count
    /*  let a = Node {
        data: 5,
        next: Some(Node {
            data: 10,
            next: None,
        }),
    };

    let b = Node {
        data: 3,
        next: Some(a),
    };

    let c = Node {
        data: 4,
        next: Some(a),
    }; */

    // Checking the count
    /*     print!(
        "counts after b and c are created = {}",
        Rc::strong_count(&a)
    );
    {
        // Increasing count in a scope
        let d = Node {
            data: 20,
            next: Some(a),
        };
        print!("counts after d is created = {}", Rc::strong_count(&a));
    }
    print!("counts d goes out of scope = {}", Rc::strong_count(&a)); */
}
