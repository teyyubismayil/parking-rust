pub struct Parking{
    c: i32
}

impl Parking {
    pub fn new(c: i32) -> Parking {
        return Parking { c }
    }
    
    pub fn count(&self) -> i32 {
        return self.c;
    }
    
    pub fn leave(&mut self) {
        self.c += 1;
    }

    pub fn reserve(&mut self) -> bool {
        if self.c > 0 {
            self.c -= 1;
            return true;
        }
        
        return false;
    }
}
