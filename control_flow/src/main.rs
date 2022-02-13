fn if_statement(x: i32) {
    if x > 30 {
        println!("It is warm");
    } else if x < 10 {
        println!("It is cold");
    } else {
        println!("It is tepid");
    }
}

fn while_and_loop() {
    let mut x = 1;

    while x < 1000 {
        x *= 2;
        if x == 64 { continue; } // Takes back to top, skips println below
        println!("x = {}", x);
    }

    let mut y = 1;
    loop  { // While true
        y *=2;
        println!("y = {}", y);

        if y == 1<<10 { break; } // Takes you out of the loop entirely
    }
}

fn for_loop() {
    for x in 1..11 {
        if x == 3 { continue; } // Go back to loop, skip println
        if x == 8 { break; } // Break out completely
        println!("x = {}", x);
    }

    // With enumeration of element
    for (pos,y) in (30..41).enumerate() {
        println!("pos = {}, and y = {}", pos, y);
    }
}

fn match_test() {
    let country_code = 5;

    let country = match country_code {
        44 => "UK",
        45 => "Sweden",
        7 => "Russia",
        1..=1000 => "unknown", // inclusive, everything from 1 to 1000 including 1000
        _ => "invalid"
    };

    println!("The country with code {} is {}",
        country_code, country);

    let x = false;
    let s = match x {
        true => "yes",
        false => "no"
    };
}

fn main() {
    // if_statement(temp);
    // while_and_loop();
    // for_loop();
    // match_test();
}