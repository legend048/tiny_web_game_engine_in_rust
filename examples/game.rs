extern "C" {
    pub fn log_number(number: usize);
}

fn main() {
    unsafe {
        log_number(4);
    }
}
