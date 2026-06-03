#[derive(PartialEq, Debug)]
pub struct USD(f32);
#[derive(PartialEq, Debug)]
pub struct GBP(f32);
#[derive(PartialEq, Debug)]
pub struct EUR(f32);

pub trait ToUSDv<F>{
    fn to_uv(&self, _: F) -> f32;
}

pub trait FromUSDv<F>{
    fn from_uv(&self, _: f32) -> F;
}

pub struct Ex {
    EUR: f32,
    GBP: f32
}

impl ToUSDv<GBP> for Ex {
    fn to_uv(&self, g:GBP) -> f32{
        (g.0 as f32) * self.GBP
    }
}

impl FromUSDv<EUR> for Ex {
    fn from_uv(&self, f: f32) -> EUR{
        EUR((f / self.EUR) as f32)
    }
}

pub trait Exchange<F,T>{
    fn convert(&self, _: F) -> T;
}

impl<E, F, T> Exchange<F,T> for E
    where E:ToUSDv<F> + FromUSDv<T>
{
    fn convert(&self, f:F) -> T{
        self.from_uv(self.to_uv(f))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let g = GBP(200.0);
        let ex = Ex{EUR: 1.16, GBP: 1.34};
        let e: EUR = ex.convert(g);
        
        println!("{:?}", e);
        assert_eq!(e, EUR(231.03448));
    }
}
