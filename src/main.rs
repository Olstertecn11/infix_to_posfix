mod utils;
use std::collections::LinkedList;

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
    let mut output: String = String::from("");
    let mut stack: LinkedList<char> = LinkedList::new();
    expression.chars().for_each(|ch| {
        if !is_op(ch, &ops){
            output.push(ch);
        }
        else if ch == '('{
            stack.push_back(ch);
        }
        else if ch == ')'{
            while stack.back().unwrap() != &'(' {
                output.push(*stack.back().unwrap());
                stack.pop_back();
            }
        }
        else{
            while !stack.is_empty() && priority(ch) <= priority(*stack.back().unwrap()){
                output.push(*stack.back().unwrap());
                stack.pop_back();
            }
            stack.push_back(ch);
        }
    });

    while !stack.is_empty(){
        output.push(*stack.back().unwrap());
        stack.pop_back();
    }
    return output;
}


fn main() {
    let operators: String = String::from("+-*/^");
    let input = utils::readline("Enter a expression: ".to_string());
    let result: String = toPosfix(input, &operators);
    utils::clear();
    println!("Result: {}", result);
    utils::pause();
    // for i in input.chars(){
    //     if is_op(i, &operators) {
    //         println!("is op: {}", i);
    //     }
    // }

}
