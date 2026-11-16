use std::io;

// to_celsius: true, from fahrenheit to celcius; false from celsius to fahrenheit
fn cal(degree: f32, to_celsius: bool) -> f32 {
    if to_celsius {
        return (degree - 32.0) * 5.0 / 9.0;
    }

    return (degree * 9.0 / 5.0) + 32.0;
}

pub fn convert() {

    println!("input two number separated by space. 1. degree; 2. 1 for fahrenheit to celsius, 0 for celsius to fahrenheit");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed input");

    let parts: Vec<&str> = input.split_whitespace().collect();

    if parts.len() >= 2 {
        let degree: f32 = parts[0].parse().expect("need a degree");
        let flag: bool = match parts[1] {
            "1" => true,
            "0" => false,
            _ => panic!("only 1 or 0"),
        };

        println!("input is: degree: {}, to celsius: {}", degree, flag);

        let ans: f32  = cal(degree, flag);

        if flag {            
            println!("fahrenheit: {} to celsius: {}", degree, ans);
            return;
        }
        
        println!("celsius: {} to fahrenheit: {}", degree, ans);
        
    } else {
        println!("need two input");
    }
}

