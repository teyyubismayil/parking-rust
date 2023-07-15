pub mod parking;

fn main() {
    let mut p = parking::Parking::new(3);
    
    loop {
        let mut operation = String::new();
        println!("Please enter operation:");
        std::io::stdin().read_line(&mut operation).unwrap();

        match operation.trim_end() {
            "count" => {
                println!("{}", p.count());
            },
            "reserve" => {
                if p.reserve() {
                    println!("Reserved");
                } else {
                    println!("No empty space");
                }
            },
            "leave" => {
                p.leave();
            },
            "bye" => {
                println!("Exiting");
                break;
            },
            _ => {
                println!("Unrecognized operation");
            }
        };
    }
}
