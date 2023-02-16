mod utils;

struct Stack<T> {
  stack: Vec<T>,
}

impl<T> Stack<T> {
  fn new() -> Self {
    Stack { stack: Vec::new() }
  }

  fn length(&self) -> usize {
    self.stack.len()
  }

  fn pop(&mut self) -> Option<T> {
    self.stack.pop()
  }

  fn push(&mut self, item: T) {
    self.stack.push(item)
  }

  fn is_empty(&self) -> bool {
    self.stack.is_empty()
  }

  fn peek(&self) -> Option<&T> {
    self.stack.last()
  }
}

fn is_op(ch: char, ops: &String)->bool{
    ops.find(ch) != None
}

fn priority(ch: char)->u32{
    if ch == '+' || ch == '-'{
        return 1;
    }
    else if ch == '*' || ch == '/' {
        return 2;
    }
    else if ch == '^'{
        return 3;
    }
    return 0;
}


fn toPosfix(expression: String, ops: &String)-> String{
    let output: String = String::from("");
    let mut stack: Stack<char> = Stack::new();
    expression.chars().for_each(|char| {
        if !is_op(char, &ops){
            output.push(char);
        }
        else if char == '('{
            stack.push(char);
        }
        else if char == ')'{

        }
    });


}


fn main() {
    let operators: String = String::from("+-*/^");
    let input = utils::readline("Enter a expression: ".to_string());
    for i in input.chars(){
        if is_op(i, &operators) {
            println!("is op: {}", i);
        }
    }

}
