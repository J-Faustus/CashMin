//! Texts to be put in the executable so there's not any stray .txt file dependencies

pub const helpText:&'static str=
"CashMin Interactive Mode Help
Enter Q or Quit to Quit, or press CTRL-C

Your actual and hypothetical balances are stored separately. Setting your balance affects your 'real' balance and resets your hypothetical balance to it.
Your hypothetical balance is used for the interactive availability checking. This enables you to check whether you can buy combinations of items
interactively instead of manually checking using the manual, if you generated one.

Current Commands:
    help:                               see this text
    make:                               makes instruction manual Zero_Track.txt (~300 kB)

    set balance:                        sets your real balance and resets your hypothetical balance to it
    reset balance:                      resets hypothetical balance to your actual balance
    check balance:                      checks your actual balance against price data to determine viability
                                            Implicitly done when setting balance

    show items:                         displays all items in categories by increasing price
         available:                     shows all categories available for purchase under the checking and nonsense restrictions
         balance:                       shows your hypothetical balance
         cart:                          shows items you bought

    buy [n: optional] [item/category]:  attempts to purchase n of an item or an item category. 
                                            If successful, deducts from your hypothetical balance. 
                                            If unsuccessful, it informs you of the failure.
    sell [n: optional] [item/category]: attempts to undo the purchase of n of an item or item category

    checklevel [optimal, debt, none]:   set the check level to ensure optimal purchasing, only a debt-free balance, or nothing

    clear:                              clears the screen
    title:                              clears the screen and prints the title";

pub const title:&'static str=
r"------------------------------------------------------------------------------------------------
                                  
           __________          __|_|___                                ___________
          //       //   /|    / _|_|__/    ||       ||   |\       /|       ||        ||       ||
         //       //   //|   / / | |       ||       ||   ||\     /||       ||        ||       ||
        //       //   // |   | | | |       ||       ||   || \   / ||       ||        ||\      ||
       //            //  |   | | | |       ||       ||   ||  \_/  ||       ||        || \     ||
      //            //   |   \ \_|_|__     ||       ||   ||       ||       ||        ||  \    ||
     //            //____|    \__|_|_ \    ||_______||   ||       ||       ||        ||   \   ||
    //            //     |       | | \ \   ||       ||   ||       ||       ||        ||    \  ||
   //       //   //      |       | | | |   ||       ||   ||       ||       ||        ||     \ ||
  //       //   //       |       | | | |   ||       ||   ||       ||       ||        ||      \||
 //       //   //        |     __|_|_/ /   ||       ||   ||       ||       ||        ||       ||
//_______//   //         |    /__|_|__/    ||       ||   ||       ||   ____||_____   ||       ||
                                 | |

------------------------------------------------------------------------------------------------";

pub const credits:&'static str=
"-------------------------------------------Credits-------------------------------------------
Brendan Herman             - like 99% of the code here
M------ C---               - came up with the idea. He had an original implementation that
                             took a \"top-down\" approach to calculating optimal combinations
D---- A-----               - greatly improved the Frobenius number algorithm";