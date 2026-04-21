//! # Utilities
//! various utilities for the interactive mode

use crate::UI::{CheckLevel, Parameters, State, texts::*};
use std::io::Write;

///prints a fancy title
pub fn print_title() {
    clear_screen();
    println!("{}", title);
}

///Check a balance against Frobenius number and unique prices
pub fn check_balance(balance: u32, FN: u32, uniquePrices: &Vec<u32>) {
    println!("\n----------\n");

    if balance > FN + 2 * uniquePrices.iter().max().expect("") {
        println!("You can buy at least any pair of items");
    } else if balance > FN + uniquePrices.iter().max().expect("") {
        println!(
            "You can certainly reach zero, but you won't necessarily be able to purchase what you want."
        );
        println!("You can buy any single item.");
        println!("Use the buy function with optimal check level to be certain of your choices.");
    } else if balance > FN {
        println!(
            "Tread Carefully. There may be only one path to success, but one is guaranteed to exist."
        );
    } else {
        println!(
            "The following text is in error, and I will fix it in the future. You may or may not be able to reach zero."
        );
        println!(
            "Use the show available function. if the nearest optimal balance is your balance, then you're fine."
        );
        //TODO: fix this so that accessible values less than the frobenius number are correctly registered as being able to reach zero
        println!(
            "There is no redemption. You failed. Womp womp.\nBingle bongle dingle dangle yikidi doo yikidi dah ping pong lippy toppy too tah."
        );
        println!("You can continue to buy optimally, but you can't reach zero anymore.");
    }
    println!("\n----------\n");
}

///Get the balance in cents
pub fn get_balance() -> u32 {
    loop {
        let mut buffer: String = String::new();
        print!("Enter current MunchMoney balance: ");
        std::io::stdout().flush();
        std::io::stdin().read_line(&mut buffer);
        if let Some(c) = buffer.trim().chars().next()
            && c == '$'
        {
            //user entered dollars explicitly
            if let Result::Ok(v) = buffer.trim().strip_prefix('$').unwrap().parse::<f32>() {
                return (100f32 * v) as u32;
            }
        } else if let Ok(v) = buffer.trim().parse::<u32>() {
            return v;
        } else if let Ok(v) = buffer.trim().parse::<f32>() {
            return (v * 100f32) as u32;
        } else {
            println!("Unable to parse {}. Please enter a valid input.", buffer);
        }
    }
}

///display items in price categories
pub fn show_items(uniquePrices: &Vec<u32>, categories: &Vec<Vec<String>>) {
    //TODO: format better
    for c in 0..uniquePrices.len() {
        println!("--- Category {}: {} cents ---", c, uniquePrices[c]);
        for item in categories.get(c).expect("").iter() {
            println!("{}", item);
        }
    }
}

///generate availability map
/// Used by the Paramters struct to determine the availability of items at different balances. Cannot therefore be made to take a Paramters struct.
pub fn avaiability_map(FN: u32, prices: &Vec<u32>) -> Vec<u128> {
    //TODO: find a way to not have the 128-bit restriction
    let mut uniquePrices: Vec<u32> = Vec::new();

    //get all prices in ascending order
    for x in prices.iter() {
        if !uniquePrices.contains(x) {
            uniquePrices.push(*x)
        }
    }
    uniquePrices.sort();

    //now compute the possibilities
    //assume the number of items is at most 128

    assert!(
        uniquePrices.len() <= 128,
        "There are too many unique price options to track. Cannot make availability map."
    );

    let mut possibilities: Vec<u128> = Vec::with_capacity((FN + uniquePrices.iter().max().expect("")) as usize);
    for _ in 0..FN + uniquePrices.iter().max().expect("") {
        possibilities.push(0);
    }
    possibilities[0] = 1; //the unit bit is for minprice

    let mut bit: u128 = 1;

    for &price in uniquePrices.iter() {
        //for all unique prices
        for i in 0..possibilities.len() - price as usize {
            if possibilities[i] > 0 {
                possibilities[i + price as usize] |= possibilities[i] | bit;
            }
        }
        bit <<= 1;
    }

    possibilities[0] = 0;

    //possibilities now explains how each attainable value can reach zero
    possibilities
}

///puts names into categories by price point
/// Used by Parameters to fully determine the financial situation, so this cannot itself use a Parameters struct
pub fn categorize(
    prices: &Vec<u32>,
    uniquePrices: &Vec<u32>,
    names: &Vec<String>,
) -> Vec<Vec<String>> {
    let mut categories: Vec<Vec<String>> = Vec::new();
    categories.reserve(uniquePrices.len());
    for _ in 0..uniquePrices.len() {
        categories.push(Vec::new());
    }

    //put names in categories
    for i in 0..names.len() {
        let category = uniquePrices
            .iter()
            .enumerate()
            .find(|t| *t.1 == prices[i])
            .expect("")
            .0;
        categories[category].push(names[i].clone());
    }
    categories
}

//show available categories
pub fn show_available_categories(state: &State, params: &Parameters) {
    match state.checking {
        CheckLevel::Optimal => {
            if state.balance as usize >= params.availability.len() {
                println!("You can buy anything");
            } else {
                match find_optimal_index(&params.availability, state.hypothetical) {
                    Some(x) => {
                        println!("Balance: {}", state.hypothetical);
                        println!("Nearest optimal balance: {x}");
                        println!("You can purchase from the following categories:");
                        for c in (0..128u8)
                            .into_iter()
                            .map(|n| (params.availability[x as usize] & (1u128 << n)) > 0)
                            .enumerate()
                            .filter(|x| x.1)
                        {
                            println!("{}", c.0);
                        }
                    }
                    None => println!("You cannot buy anything."),
                }
            }
        }
        CheckLevel::Debt => {
            match (0..params.uniquePrices.len())
                .rev()
                .into_iter()
                .map(|t| (t, params.uniquePrices[t] < state.hypothetical))
                .find(|t| t.1)
            {
                Some((i, _)) => {
                    println!("You can buy anything with category number at most {}.", i)
                }
                None => println!("You cannot buy anything."),
            }
        }
        CheckLevel::Neither => {
            println!("You can buy anything since you chose not to care about sensibility.")
        }
    }
}

///finds the largest possible optimal expenditure for a given price point
pub fn find_optimal_index(availability: &Vec<u128>, balance: u32) -> Option<u32> {
    match (0..=balance)
        .rev()
        .into_iter()
        .map(|i| (i, availability[i as usize] > 0))
        .find(|x| x.1)
    {
        Some((x,_)) => Some(x as u32),
        None => None,
    }
}

pub fn clear_screen() {
    print!("\x1B[2J\x1B[3J\x1B[1;1H");
}
