use crate::models::car::{Car, CarList};

#[allow(dead_code)]
pub fn car_fixture(id: i32) -> Car {
    Car {
        id,
        name: String::from("ferrari"),
        color: Some(String::from("black")),
        year: Some(1980),
    }
}

#[allow(dead_code)]
pub fn cars_fixture(num: i32) -> CarList {
    let mut cars = vec![];
    for i in 1..num + 1 {
        cars.push(car_fixture(i));
    }
    CarList {
        data: cars,
        total: (num * 9) as i64,
    }
}
