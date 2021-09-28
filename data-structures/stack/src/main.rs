fn main() {
    let mut stck: [i32; 10] = [0; 10]; // stack initialization
    let mut top:usize = 0;
    
    push(&mut stck, &mut top, 5);
    let ret = pop(&mut stck, &mut top); 
    println!("pop: {}", ret);
    push(&mut stck, &mut top, 6);
    let ret = pop(&mut stck, &mut top);
    println!("pop: {}", ret);
    
    println!("Hello, world!");
}

fn empty(top: usize) -> bool {
    if top == 0 {
        return true;
    }

    return false;
}

fn full(s: &[i32;10], top: usize) -> bool 
{
    if s.len() == top {
        return true;
    }

    return false;
}

fn push(s: &mut[i32;10], top: &mut usize, new: i32) {
    if full(&s, *top) {
        panic!("full");
    }
    *top = *top + 1;
    s[*top] = new;
}

fn pop(s: &mut [i32;10], top: &mut usize) -> i32 {
    if empty(*top) {
        panic!("empty");
    }
    let ret = s[*top];
    s[*top] = 0;
    *top = *top - 1;
    ret
}
