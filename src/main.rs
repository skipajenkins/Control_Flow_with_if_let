
#[derive(Debug)]

enum Ustate{
    Alabama,
    Alaska,
    //Other states
}

impl Ustate{
    fn existed_in(&self,year:u16) -> bool{
        match self{
            Ustate::Alabama => year >= 1819,
            Ustate::Alaska => year >= 1959,
            // and so on
        }
    }
}

enum Coin{
    Nickel,
    Quarter(Ustate),
}

fn describe_state_quarter(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}


fn main() {
    // let config_max = Some(3u8);
    // match config_max{
    //     Some(max) => println!("The maximum is configured to be {max}"),
    //     _=>(),
    // }
    //The above ccode can be rewritten using if let
    let config_max = Some(3u8);
    if let Some(max) = config_max{
        println!("The maximum is configured to be  {max}");
    }

    //Writing the above using if else statement to see if there is a
    //difference between if let and just usin regular if.

    // let config_max = Some(3u8);
    // if Some(max) = config_max{
    //     println!("The maximum is configured to be  {max}");
    // }

    //Ok the above doesn't work because using if else assumes the 
    //variable likew max have be declared (let max;)
    //But with the if let it's another approach to pattern matching.

    let mut count = 6;
    let coin = Coin::Nickel;
    match coin{
        Coin::Quarter( ref state) => println!("State quarter from {state:?}!"),
        _ => count +=1,
    }
    
    //Or we could use an if let and else expression rather than a match.
    let mut count_0 = 0;
    if let Coin::Quarter(state) = coin{
        println!("State quarter from {state:?}!");
    } else{
        count_0 += 1;
    }
    //Only Error from the above code is the Coin and coin are undeclared.
    //To fix this we introduce an enum Coin with variant Quarter
    // rather just (state) to stop the value from being drop after it
    //has been moved we use ref to reference it and not move the actual value
}
