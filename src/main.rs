use std::str::FromStr;
mod temperature;
use temperature::{Temperature, TemperatureType};

fn main() {
    let group = std::env::args().nth(1).unwrap().to_string();
    let input_type = std::env::args().nth(2).unwrap().to_string();
    let output_type = std::env::args().nth(3).unwrap().to_string();
    let value = std::env::args().nth(4).unwrap().to_string();

    let group_type: GroupType =
        GroupType::from_str(group.as_str()).expect("Invalid group type, check documentation");
    match group_type {
        GroupType::HELP => println!("Help"),
        GroupType::TEMPERATURE => {
            let input_temp_type = TemperatureType::from_str(input_type.as_str()).expect("Invalid temperature type {}");
            let output_temp_type = TemperatureType::from_str(output_type.as_str()).expect("Invalid temperature type {}");
            let input_value = value.parse::<f32>().unwrap();
            let mut t = Temperature::new(input_temp_type, output_temp_type, input_value);
            let output_value = t.process();
            println!("{}", output_value.output.unwrap());
        },
        GroupType::TEXT => println!("Text"),
        GroupType::UNIT => println!("Unit"),
        GroupType::ENCRYPT => println!("Encrypt"),
        GroupType::CURRENCY => println!("Currency"),
        _ => panic!("Unidentified group for conversion"),
    }
}

enum GroupType {
    HELP,
    TEMPERATURE,
    TEXT,
    UNIT,
    ENCRYPT,
    CURRENCY,
}

impl FromStr for GroupType {
    type Err = ();
    fn from_str(input: &str) -> Result<GroupType, Self::Err> {
        match input.to_lowercase().as_str() {
            "h" | "help" => Ok(GroupType::HELP),
            "temp" | "temperature" => Ok(GroupType::TEMPERATURE),
            "encrypt" | "enc" => Ok(GroupType::ENCRYPT),
            "text" => Ok(GroupType::TEXT),
            "unit" => Ok(GroupType::UNIT),
            "curr" | "currency" => Ok(GroupType::CURRENCY),
            _ => Err(()),
        }
    }
}