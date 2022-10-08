// function pointers
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);

    let nums = vec![1, 2, 3];
    let _num_str: Vec<String> = nums.iter().map(| i | i.to_string()).collect();
    let _num_str2: Vec<String> = nums.iter().map(ToString::to_string).collect();

    enum Status {
        Value(u32),
        Stop
    }

    let _statuses: Vec<Status> = (0_u32..10).map(Status::Value).collect();
}

// returning closures
fn main() {}

fn returns_closure() -> dyn Fn(i32) -> i32 {
    | x | x + 1
}

fn returns_closure(y: i32) -> Box<dyn Fn(i32) -> i32> {
    if y > 0 {
        Box::new(move | x | y + x)
    } else {
        Box::new(move | x | y - x)
    }
}