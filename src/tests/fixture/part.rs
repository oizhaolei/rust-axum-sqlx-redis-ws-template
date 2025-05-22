use crate::models::part::{Part, PartList};

#[allow(dead_code)]
pub fn part_fixture(id: usize) -> Part {
    Part {
        id: id as i32,
        car_id: Some(1),
        name: String::from("alternator"),
    }
}

#[allow(dead_code)]
pub fn parts_fixture(num: usize) -> PartList {
    let mut parts = vec![];
    for i in 1..num + 1 {
        parts.push(part_fixture(i));
    }
    PartList {
        data: parts,
        total: (num * 9) as i64,
    }
}
