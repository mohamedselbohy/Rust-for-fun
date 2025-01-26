fn main() {
    let mut msg: String = String::from("world");
    println!(
        "Hello, world! {}",
        another_function(5, &mut msg).unwrap()
    );
    //msg.push('s');
    msg = "blablablabla".to_string();
    match msg.as_str() {
        "world" => println!("where is your s?"),
        "worlds" => println!("there it is!"),
        other => println!("I don't know what you mean by {}", other),
    }
    println!("{}",msg);
}

fn another_function(var: i32, var2: &mut String) -> Result<String, String> {
    if var < 0 {
        return Err(String::from("Error"));
    } else {
        return Ok(var2.to_string());
    }
}
