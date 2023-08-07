use std::io;

fn main() {
    let todo_list = Vec::new();
    let mut todo_manager = TodoManager { todos: todo_list };

    loop {
        let mut string = String::from("");

        match io::stdin().read_line(&mut string) {
            Ok(_) => {
                let new_todo = Todo {
                    id: todo_manager.todos.len() as u32,
                    task: string.trim().to_string(),
                    completed: false,
                };

                todo_manager.todos.push(new_todo);
                println!("list of todos: {:?}", todo_manager.todos);
                return;
            }
            Err(_) => todo!(),
        }
    }
}

#[derive(Debug)]
struct TodoManager {
    todos: Vec<Todo>,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Todo {
    id: u32,
    task: String,
    completed: bool,
}

#[allow(dead_code)]
impl TodoManager {
    fn update(&mut self, id: u32, updated_todo: Todo) {
        self.todos.insert(id as usize, updated_todo);
    }

    fn get(&self, id: u32) {
        println!("your todo : {:?}", self.todos.get(id as usize));
    }

    fn delete(&mut self, id: u32) {
        self.todos.remove(id as usize);
    }
}
