#![allow(nonstandard_style, unused_must_use)]

use crate::FileIO::get_data;

#[cfg(test)]
mod test;

mod math;
mod UI;
mod FileIO;

fn main() {    
    //read file
    let (prices, names)=match get_data(){
        Result::Ok((prices, names))=>(prices, names),
        _=>panic!(),
    };
    //we want this data to never be overwritten, so we use immutable variables

    let params=UI::Parameters::New(prices, names);
    let mut state=UI::State{
        cart:Vec::new(),balance:0,hypothetical:0, checking:UI::CheckLevel::Optimal};
    UI::do_interactive(&mut state, &params);
}
