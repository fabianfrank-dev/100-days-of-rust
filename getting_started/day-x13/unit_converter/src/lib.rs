enum Units{
    Kilometres,
    Metres,
    Miles,
    Yards,
    Feet
}

fn convert(value: f64, from: Units, to: Units) -> Result<f64, String>{
    match (from, to){
        (Units::Kilometres, Units::Metres) => {
            let converted_value = value * 1000.0;
            Ok(converted_value)
        },
        (Units::Metres, Units::Yards) => {
            let converted_value = value * 1.09;
            Ok(converted_value)
        },
        (Units::Miles, Units::Feet) => {
            let converted_value = value * 5280.0;
            Ok(converted_value)
        }
        (Units::Yards, Units::Feet) => {
            let converted_value = value * 3.0;
            Ok(converted_value)
        },
        _ => {
            Err("Unsupported conversion".to_string())
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(convert(12.0, Units::Kilometres, Units::Metres), Ok(12000.0));
        assert_eq!(convert(1.0, Units::Metres, Units::Yards), Ok(1.09));
        assert_eq!(convert(1.0, Units::Miles, Units::Feet), Ok(5280.0));
        assert_eq!(convert(12.0, Units::Yards, Units::Metres), Err("Unsupported conversion".to_string()));
    }
}
