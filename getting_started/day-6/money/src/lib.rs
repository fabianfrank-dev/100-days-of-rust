#[derive(PartialEq, Debug, Clone)]
pub struct USD (f64);
#[derive(PartialEq, Debug, Clone)]
pub struct GBP (f64);
#[derive(PartialEq, Debug, Clone)]
pub struct EUR (f64);

pub trait ToUSD{
    fn to_usd(&self) -> USD;
    fn convert<T:FromUSD>(&self) -> T{
        T::from_usd(&self.to_usd())
    }
}
pub trait FromUSD{
    fn from_usd(u: &USD) -> Self;
}

impl ToUSD for GBP{
    fn to_usd(&self) ->USD {
        USD(self.0 * 1.34)
    }
}
impl FromUSD for EUR{
    fn from_usd(u: &USD) -> Self{
        EUR(u.0 * 0.86)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let g = GBP(200.0);
        let u = g.to_usd();      
        let e = EUR::from_usd(&u);

        assert_eq!(u, USD(268.0));
        assert_eq!(e, EUR(230.48));

        let e2:EUR = g.convert();
        assert_eq!(e2, e);
    }
}
