

pub fn print(instances: &Vec<(usize, &str)>){
    for instance in instances{
        println!("{}: {}", instance.0, instance.1);
    }
}