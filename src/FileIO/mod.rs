//! FileIO

use std::fs::File;
use std::io::{Read,Write};

///makes a manual
pub fn make_manual(categories: &Vec<Vec<String>>, possibilities: &Vec<u128>, prices: &Vec<u32>) {
    println!("Computing instruction manual file");
    println!("Zero_Track.txt");

    let mut uniquePrices: Vec<u32> = Vec::new();

    //make a file detailing all allowable purchases
    //file should enumerate possible items in ascending price order. Items of the same price should have the same number.
    //then, from FN+max(prices) to zero, all attainable values should show which items may be purchased and what the resulting balance will be

    //get all prices in ascending order
    for x in prices.iter() {
        if !uniquePrices.contains(x) {
            uniquePrices.push(*x)
        }
    }
    uniquePrices.sort();

    //now, construct file

    let mut file = File::create("Zero_Track.txt").expect("Could not write file.");

    file.write(b"----------Price Categories----------\n");
    for (i, category) in categories.iter().enumerate() {
        //categories moved. will not be used again
        file.write(format!("{} -- {} cents\n", i, uniquePrices[i]).as_bytes());
        for name in category {
            file.write(name.as_bytes());
            file.write(b"\n");
        }
        file.write(b"\n");
    }

    file.write(b"\n\n");
    file.write(b"----------Permissible Purchase Paths----------\n");

    for balance in (0..possibilities.len()).rev() {
        if possibilities[balance] > 0 {
            //for all possibilities
            let mut bit = 1;
            file.write(format!("{} cents:\n", balance).as_bytes());
            let mut option = 0;
            while bit <= possibilities[balance] {
                if possibilities[balance] & bit > 0 {
                    file.write(
                        format!(
                            "\t{}->{} cents\n",
                            option,
                            balance as u32 - uniquePrices[option]
                        )
                        .as_bytes(),
                    );
                }
                bit <<= 1;
                option += 1;
                // println!("{}",bit);
            }
            file.write(b"\n");
        }
    }

    println!("Done");
}

///gets data from the file
pub fn get_data() -> Result<(Vec<u32>, Vec<String>), String> {
    let mut file = File::open("prices.csv").map_err(|x| x.to_string())?;
    let mut buffer = String::new();
    let mut names:Vec<String>=Vec::new();
    let mut prices:Vec<u32>=Vec::new();
    file.read_to_string(&mut buffer)
        .map_err(|x| x.to_string())?;
    let lines = buffer.split('\n');
    for line in lines {
        //interpret name and price
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() != 2 {
            break;
        }
        names.push(String::from(parts[0]));
        prices.push(
            (String::from(parts[1])
                .trim()
                .parse::<f32>()
                .map_err(|x| x.to_string())?
                * 100_f32) as u32,
        );
    }
    Result::Ok((prices, names))
}

