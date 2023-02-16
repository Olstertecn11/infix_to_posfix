mod utils;



fn is_op(ch: char, ops: String)->bool{
    ops.find(ch) != None
}



fn main() {
    let operators: String = String::from("+-*/^");
    let input = utils::readline("Enter a expression: ".to_string());
    for i in input.chars(){
        if is_op(i, operators) {
            println!("is op");
        }
    }

}
