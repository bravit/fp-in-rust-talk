use frunk::Validated;
use frunk::prelude::*;
use frunk_core::{HList, hlist_pat};

#[derive(PartialEq, Eq, Debug)]
struct Person {
    age: i32,
    name: String,
}

fn get_name() -> Result<String, String> {
    Ok("James".to_owned())
}

fn get_age() -> Result<i32, String> {
    Ok(32)
}

#[test]
fn test_validate() {
    let v: Validated<HList!(String, i32), String> = get_name().into_validated() + get_age();
    let person = v.into_result()
        .map(|hlist_pat!(name, age)| {
            Person {
                name,
                age,
            }
        });

    assert_eq!(person.unwrap(),
               Person {
                   name: "James".to_owned(),
                   age: 32,
               });
}

fn main() {}