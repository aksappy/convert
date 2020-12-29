use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum TemperatureType {
    CELSIUS,
    KELVIN,
    FAHRENHEIT,
}
impl FromStr for TemperatureType {
    type Err = ();
    fn from_str(input: &str) -> Result<TemperatureType, Self::Err> {
        match input.to_lowercase().as_str() {
            "c" | "cel" | "celsius" => Ok(TemperatureType::CELSIUS),
            "k" | "kel" | "kelvin" => Ok(TemperatureType::KELVIN),
            "f" | "fah" | "fahrenheit" => Ok(TemperatureType::FAHRENHEIT),
            _ => Err(()),
        }
    }
}

pub struct Temperature {
    input: f32,
    pub output: Option<f32>,
    input_type: TemperatureType,
    output_type: TemperatureType,
}

impl Temperature {
    
    pub fn new(input_type: TemperatureType, output_type: TemperatureType, value: f32) -> Self {
        return Temperature {
            input: value,
            output: Some(0.0),
            input_type: input_type,
            output_type: output_type,
        };
    }
    pub fn process(&mut self) -> &Self {
        let mut output : f32 = self.input;
        match self.input_type {
            TemperatureType::CELSIUS => {
                match self.output_type {
                    TemperatureType::KELVIN => output = self.input + 273.15 as f32,
                    TemperatureType::FAHRENHEIT => output = self.input * 9 as f32 / 5 as f32 + 32 as f32,
                    _ => panic!("Invalid target type, see help for more info")
                }
            },
            TemperatureType::KELVIN => {
                match self.output_type {
                    TemperatureType::CELSIUS => output = self.input - 273.15 as f32,
                    TemperatureType::FAHRENHEIT => output=  (self.input + 32 as f32) / 5 as f32 * 9 as f32 - 273.15 as f32,
                    _ => panic!("Invalid target type, see help for more info")
                }
            },
            TemperatureType::FAHRENHEIT => {
                match self.output_type {
                    TemperatureType::KELVIN => output =  (self.input - 32 as f32) * 5 as f32 / 9 as f32 + 273.15 as f32,
                    TemperatureType::CELSIUS => output =  self.input  / 5 as f32 * 9 as f32,
                    _ => panic!("Invalid target type, see help for more info")
                }
            },
            _ => panic!("Invalid target type, see help for more info"),
        };
        self.output = Some(output);
        return self;
    }
}