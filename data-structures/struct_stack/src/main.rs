struct Stack {
    stack: [i32; 10],
    top: usize,
}


fn main() {
    let mut s = Stack {
        stack: [0; 10],
        top: 0,
    };

    push(&mut s, 5);
    println!("pop value: {}", pop(&mut s)); 
}

fn push(s: &mut Stack, val: i32){
    if s.top == 10 {
        panic!("stack is full");
    }

    s.stack[s.top] = val;
    s.top += 1;
}

fn pop(s: &mut Stack) -> i32 {
    if s.top == 0 {
        panic!("Stack is empty!");
    }

    s.top -= 1;
    let pop_value = s.stack[s.top];
    pop_value
}
