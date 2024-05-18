use super::Number;

pub fn reduce(mut upper: i64, mut lower: i64) -> Number {
    let gcd_result = Number::gcd(upper, lower);
    upper /= gcd_result;
    lower /= gcd_result;
    return Number::Fraction(upper, lower);
}
