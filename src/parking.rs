use std::sync::{Mutex};

pub struct Parking{
    c: i32,
    m: Mutex<i8>
}

impl Parking {
    pub fn new(c: i32) -> Parking {
        return Parking { c, m: Mutex::new(0) }
    }
    
    pub fn count(&self) -> i32 {
        return self.c;
    }
    
    pub fn leave(&mut self) {
        let _guard = self.m.lock().unwrap();
        
        self.c += 1;
    }

    pub fn reserve(&mut self) -> bool {
        let _guard = self.m.lock().unwrap();

        if self.c > 0 {
            self.c -= 1;
            return true;
        }
        
        return false;
    }
}
