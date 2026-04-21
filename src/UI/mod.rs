//! UI
//! Handles all elements of user interface, abstracted behind the single function do_interactive()

mod command;
mod texts;
pub mod utilities;

use crate::math::{frobenius_number, gcd_vec};
use crate::{
    FileIO,
    UI::texts::{credits, title},
};
use command::*;
use std::collections::HashMap;
use utilities::*;

pub struct State {
    pub cart: Vec<CartItem>,
    pub checking: CheckLevel,
    pub balance: u32,
    pub hypothetical: u32,
}

#[derive(PartialEq, Clone, Copy)]
pub enum CartItem {
    Item(u32),
    Category(u32),
}

pub struct Parameters {
    pub prices: Vec<u32>,
    pub uniquePrices: Vec<u32>,
    pub names: Vec<String>,
    pub availability: Vec<u128>,
    pub categories: Vec<Vec<String>>,
    pub FN: u32,
    NameMap: HashMap<String, (usize, u32)>,
}

impl Parameters {
    pub fn New(prices: Vec<u32>, names: Vec<String>) -> Self {
        //TODO: Check whether insertion sort is better here
        let mut uniquePrices: Vec<u32> = Vec::new();
        for x in prices.iter() {
            if !uniquePrices.contains(x) {
                uniquePrices.push(*x)
            }
        }
        uniquePrices.sort();

        //asserts the dataset is not empty and has unit gcd
        //TODO: allow for non-unit gcd
        assert!(
            gcd_vec(&uniquePrices).expect("Cannot work with empty dataset.") == 1,
            "It is not generally possible to reach zero with this dataset. Future versions will allow for this."
        );
        let FN = frobenius_number(&prices)
            .expect("Error occurred in generating a Frobenius number");
        println!("Frobenius Number: {}", FN);

        let categories = utilities::categorize(&prices, &uniquePrices, &names);
        let availability = utilities::avaiability_map(FN, &prices);
        let mut NameMap: HashMap<String, (usize, u32)> = HashMap::new();
        for i in 0..names.len() {
            NameMap.insert(names[i].clone(), (i, prices[i]));
        }
        Parameters {
            prices,
            uniquePrices,
            names,
            availability,
            categories,
            NameMap,
            FN,
        }
    }

    pub fn lookup(&self, name:&str)->Option<&(usize, u32)>{
        self.NameMap.get(name)
    }
}

//TODO: make a checklevel where the program optimises for minimum wasted value:
//  i.e. decide if and how much real money should be spent in order to reach zero if it is
//  otherwise impossible in order to minimize the amount of money that is wasted by not reaching zero on munch money
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CheckLevel {
    Optimal,
    Debt,
    Neither,
}

pub fn do_interactive(state: &mut State, params: &Parameters) {
    print_title();
    state.balance = get_balance();
    println!("You entered a balance of {} cents.", state.balance);

    state.hypothetical = state.balance;

    println!("Beginning interactive mode");

    loop {
        //TODO: make better command interpretation
        //TODO: input item, set item price, buy custom,
        //TODO: record hypothetical spending so the user can get a shopping list
        //TODO: show items more readably and beautifully
        match CommandType::fromString(get_command().to_lowercase().trim()) {
            CommandType::Quit => {
                return;
            }
            CommandType::Make => {
                FileIO::make_manual(&params.categories, &params.availability, &params.prices)
            }
            CommandType::Set => {
                state.balance = get_balance();
                state.hypothetical = state.balance;
                println!("You entered a balance of {} cents.", state.balance);
                check_balance(state.balance, params.FN, &params.uniquePrices);
            }
            CommandType::Reset => state.hypothetical = state.balance,
            CommandType::Check => {
                check_balance(state.hypothetical, params.FN, &params.uniquePrices)
            }
            CommandType::Show(args) => {
                show(args, state, params);
            }
            CommandType::Level(arg) => {
                level(arg, state);
            }
            CommandType::Clear => clear_screen(),
            CommandType::Title => {
                print_title();
            }
            CommandType::Help => println!("{}", texts::helpText),
            CommandType::Buy(args) => {
                command::buy(args, state, params);
            }
            CommandType::Sell(args) => {
                command::sell(args, state, params);
            }
            CommandType::Credits => {
                clear_screen();
                println!("{}", title);
                println!("{}", credits);
            }
            _ => println!("Unknown command"),
        }
    }
}
