use std::panic;
#[derive(Debug)]
struct MemBudget {
    remaining_bytes: usize,
}
impl MemBudget {
    fn new(size: usize) -> MemBudget {
        match size {
            0..=1024 => MemBudget {
                remaining_bytes: size,
            },
            other => panic!("Memory budget of {} too large", other),
        }
    }
    fn allocate(&mut self, bytes: usize) -> Result<(), &'static str>{
        if self.remaining_bytes >= bytes{
            self.remaining_bytes -= bytes;
            return Ok(());
        }
        return Err("Out of memory");
    }
}
fn main() {
    panic::set_hook(Box::new(|_| {
        println!("You scared rusty!");
    }));
    let budget = panic::catch_unwind(|| MemBudget::new(512)); // 1KB
    match budget {
        Ok(mut b) => {
            match MemBudget::allocate(&mut b, 24) {
                Ok(_) => {println!("Allocated bytes {:?}", b);},
                Err(e) => {println!("Error: {:?}", e);}
            }
        },
        Err(e) => {
            println!("Error: {:?}", e.downcast::<String>().unwrap());
        }
    }
}
