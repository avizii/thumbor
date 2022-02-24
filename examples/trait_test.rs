use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
struct Zoo {
    name: String,
}

impl From<String> for Zoo {
    fn from(name: String) -> Self {
        Self { name }
    }
}

#[derive(Debug)]
struct GreaterThanZero(i32);

impl TryFrom<i32> for GreaterThanZero {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value <= 0 {
            Err("GreaterThanZero only accepts value superior than zero!")
        } else {
            Ok(GreaterThanZero(value))
        }
    }
}

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s
            .trim_matches(|p| p == '(' || p == ')')
            .split(',')
            .collect();

        let x = coords[0].parse::<i32>()?;
        let y: i32 = coords[1].parse()?;

        Ok(Point { x, y })
    }
}

fn main() {
    let s = "National Zoo".to_string();
    let z1 = Zoo::from(s);

    let s = "FuJin".to_string();
    let z2: Zoo = s.into();

    println!("z1: {:?}, z2: {:?}", z1, z2);

    let num: Result<GreaterThanZero, usize> = (32_i32)
        .try_into()
        .map_err(|x: &str| x.len());

    println!("number try from : {:?}", num.unwrap());

    let try_success_num = GreaterThanZero::try_from(32_i32);
    assert!(try_success_num.is_ok());

    let try_error_num = GreaterThanZero::try_from(-32_i32);
    assert!(try_error_num.is_err());

    let p = Point::from_str("(1,2)").unwrap();
    assert_eq!(p, Point { x: 1, y: 2 });

    let p1 = "(26,170)".parse::<Point>().unwrap();
    let p2: Point = "(170,26)".parse().unwrap();

    println!("p1: {:?}, p2: {:?}", p1, p2);
}
