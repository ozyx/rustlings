// test1.rs
// This is a test for the following sections:
// - Variables
// - Functions

// Mary is buying apples. One apple usually costs 2 dollars, but if you buy
// more than 40 at once, each apple only costs 1! Write a function that calculates
// the price of an order of apples given the order amount. No hints this time!

// Put your function here!
// fn ..... {

// Don't modify this function!
#[test]
fn verify_test() {
    let price1 = calculate_price(55);
    let price2 = calculate_price(40);

    assert_eq!(price1, 55);
    assert_eq!(price2, 80);
}

fn calculate_price(amount: i32) -> i32 {
    if(amount > 40) {
        amount
    } else {
        amount * 2
    }
}
