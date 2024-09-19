fn main() {

    let transfer = define_shit();
    
    show_transfer_amount();
    set_transfer_amount(10000);
    show_transfer_amount();
}

pub fn set_transfer_amount(amount: u64) -> u64 {
    let mut a: u64 = capture();
    a = amount;
    return a;
}

pub fn show_transfer_amount() -> u64 {
    let a = capture();
    return a;
}

pub fn define_shit() -> u64 {
    let mut transfer_amount: u64 = 50000;
    return transfer_amount;
}

fn capture() -> u64 { || {
    let a: u64 = &transfer;
    return a
} }