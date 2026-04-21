//! # Command
//! Module for functions performing interactive commands

use crate::UI::Parameters;
use crate::UI::utilities::{show_available_categories, show_items};

use super::CartItem;
use super::CheckLevel;
use super::State;
use super::utilities::find_optimal_index;
use std::io::{self, Write};
use std::u32;

///get input for interactive mode
pub fn get_command() -> String {
    print!("(C$HMN)>> ");
    io::stdout().flush();
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer);
    buffer
}

///command type enum
pub enum CommandType<'a> {
    Show(&'a str),
    Buy(&'a str),
    Sell(&'a str),
    Help,
    Make,
    Set,
    Reset,
    Check,
    Clear,
    Title,
    Level(&'a str),
    Quit,
    Credits,
    Unknown,
}

use CommandType::*;
impl<'a> CommandType<'a> {
    ///Find the command type for a command string
    pub fn fromString(cmd: &'a str) -> Self {
        if cmd.starts_with("show") {
            Show(cmd.strip_prefix("show").unwrap().trim())
        } else if cmd.starts_with("buy") {
            Buy(cmd.strip_prefix("buy").unwrap().trim())
        } else if cmd.starts_with("sell") {
            Sell(cmd.strip_prefix("sell").unwrap().trim())
        } else if cmd.starts_with("help") {
            Help
        } else if cmd.starts_with("make") {
            Make
        } else if cmd.starts_with("set") {
            Set
        } else if cmd.starts_with("reset") {
            Reset
        } else if cmd.starts_with("checklevel") {
            Level(cmd.strip_prefix("checklevel").unwrap().trim())
        } else if cmd.starts_with("clear") {
            Clear
        } else if cmd.starts_with("title") {
            Title
        } else if cmd.starts_with("check") {
            Check
        } else if cmd.starts_with("q") {
            Quit
        } else if cmd.starts_with("credits") {
            Credits
        } else {
            Unknown
        }
    }
}

///buy command
pub fn buy(args: &str, state: &mut State, params: &Parameters) {
    //deduce numbers
    let (arg, number) = if let Result::Ok(n) = args.split(' ').next().unwrap().parse::<u32>() {
        (
            args.strip_prefix(args.split(' ').next().unwrap())
                .unwrap()
                .trim(),
            n,
        )
    } else {
        (args, 1)
    };

    //TODO: Fix this
    if number != 1 {
        println!("Currently, purchasing more than one at a time is not supported")
    }

    //convert the argument to a category and CartItem
    //Since the availability map can't handle more than 128 items, a u8 is sufficient to distinguish their categories
    let (cat, prospectiveItem) = if let Result::Ok(x) = arg.parse::<u8>() {
        (x, CartItem::Category(x as u32))
    } else if let Some((x, _)) = params //TODO: use params.lookup here
        .names
        .iter()
        .enumerate()
        .find(|t| t.1.to_ascii_lowercase() == arg)
    {
        //x is the index of the name in the list
        (
            //TODO: Binary search is faster O(log n) vs O(n) since uniquePrices is sorted
            params
                .uniquePrices
                .iter()
                .enumerate()
                .find(|t| *t.1 == params.prices[x]) 
                .expect("")
                .0 as u8,
            CartItem::Item(x as u32),
        )
    } else {
        (u8::MAX, CartItem::Category(u32::MAX))
    };

    //TODO: DRY this out
    if cat as usize >= params.uniquePrices.len() {
        //Handle cases in which the category does not exist
        println!("{} isn't a recognized item or category.", cat);
    } else if state.hypothetical as usize > params.availability.len() - 1 {
        //hypothetical balance is more than sufficient
        //no check rules apply
        state.hypothetical -= params.uniquePrices[cat as usize];
        println!(
            "You can buy this item. You would have {} cents left afterwards.",
            state.hypothetical
        );
        state.cart.push(prospectiveItem);
    } else {
        match state.checking {
            CheckLevel::Optimal => {
                //For optimality, find the largest index with nonzero availability map value
                if let Some(optimal_index) =
                    find_optimal_index(&params.availability, state.hypothetical)
                {
                    if params.availability[optimal_index as usize] & (1 << cat) > 0 {
                        //hypothetical balance is sufficient
                        state.hypothetical -= params.uniquePrices[cat as usize];
                        println!(
                            "You can buy this item. You would have {} cents left afterwards.",
                            state.hypothetical
                        );
                        state.cart.push(prospectiveItem);
                    } else {
                        //hypothetical balance is insufficient
                        println!(
                            "You can't buy this item. You have {} cents still.",
                            state.hypothetical
                        );
                    }
                } else {
                    //nothing is purchasable
                    println!(
                        "You cannot purchase anything. You still have {} cents.",
                        state.hypothetical
                    );
                }
            }
            CheckLevel::Debt => {
                if params.uniquePrices[cat as usize] < state.hypothetical {
                    state.hypothetical -= params.uniquePrices[cat as usize];
                    println!(
                        "You can buy this item. You would have {} cents afterwards.",
                        state.hypothetical
                    );
                    state.cart.push(prospectiveItem);
                } else {
                    println!(
                        "You cannot buy this item. You still have {} cents.",
                        state.hypothetical
                    );
                }
            }
            CheckLevel::Neither => {
                state.hypothetical = state
                    .hypothetical
                    .wrapping_sub(params.uniquePrices[cat as usize]);
                println!(
                    "Regardless of whether you can or cannot buy this item, you managed to anyway.
Your balance may have wrapped around to an unfathomably large value.
You now have {} cents.",
                    state.hypothetical
                );
                state.cart.push(prospectiveItem);
            }
        }
    }
}

///show command
pub fn show(arg: &str, state: &mut State, params: &Parameters) {
    println!("{}", arg);
    match arg {
        "items" => {
            show_items(&params.uniquePrices, &params.categories);
        }
        "balance" => {
            println!("{} cents", state.hypothetical)
        }
        "available" => {
            show_available_categories(state, params);
        }
        "cart" => {}
        _ => {}
    }
}

///checklevel command
pub fn level(arg: &str, state: &mut State) {
    state.checking = match arg {
        "optimal" => CheckLevel::Optimal,
        "debt" => CheckLevel::Debt,
        _ => CheckLevel::Neither,
    };
}

pub fn sell(args: &str, state: &mut State, params: &Parameters) {
    //deduce numbers
    let (arg, number) = if let Result::Ok(n) = args.split(' ').next().unwrap().parse::<u32>() {
        (
            args.strip_prefix(args.split(' ').next().unwrap())
                .unwrap()
                .trim(),
            n,
        )
    } else {
        (args, 1)
    };

    //TODO: Fix this
    if number != 1 {
        println!("Currently, purchasing more than one at a time is not supported")
    }

    //find what the argument is
    let thing = if let Some((n, _)) = params
        .names
        .iter()
        .map(|n| n.to_ascii_lowercase() == arg)
        .enumerate()
        .find(|t| t.1)
    {
        CartItem::Item(n as u32)
    } else if let Result::Ok(n) = arg.parse::<u32>() {
        CartItem::Category(n)
    } else {
        println!("Item or category unrecognized");
        return;
    };
    match thing {
        CartItem::Item(n) => {
            if let Some((found, _)) = state.cart.iter().enumerate().rev().find(|t| *t.1 == thing) {
                //try removing an exact item match first
                state.cart.remove(found);
                state.hypothetical += params.prices[n as usize];
                println!(
                    "Item successfully removed from cart. You now have {} cents.",
                    state.hypothetical
                );
            } else {
                //if that fails, try removing a category match and inform the user
                if let Some((a, _)) = state
                    .cart
                    .iter()
                    .map(|c| match *c {
                        CartItem::Item(n) => n,
                        _ => 1,
                    })
                    .enumerate()
                    .rev()
                    .find(|t| params.prices[t.1 as usize] == params.prices[n as usize])
                {
                    let cat = match state.cart[a] {
                        CartItem::Category(b) => b,
                        _ => 0,
                    };
                    println!("Could not find item, but found suitable replacement: {}", 0);
                } else {
                    println!("Could not find item or suitable substitute in your cart.");
                }
            }
        }
        CartItem::Category(n) => {
            //find the last item in this category and remove it
        }
    }
}
