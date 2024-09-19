pub struct Variables {
    transfer_amount: u64,
    transfer_fee: u64,
}

fn main() {
    let mut tester: Variables = Variables {
        transfer_amount: 50000,
        transfer_fee: 100,
    };

    show_transfer_amount(&mut tester);
    set_transfer_amount(10000, &mut tester);
    show_transfer_amount(&mut tester);
}

pub fn set_transfer_amount(amount: u64, tester: &mut Variables) -> u64 {
    tester.transfer_amount = amount;
    println!("{}", tester.transfer_amount);
    return tester.transfer_amount;
}

pub fn show_transfer_amount(tester: &Variables) -> u64 {
    println!("{}", tester.transfer_amount);
    return tester.transfer_amount;
}
