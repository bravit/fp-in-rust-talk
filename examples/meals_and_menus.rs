use crate::DietaryRestriction::{Carnivore, Omnivore, Vegetarian};

#[derive(Debug, PartialEq)]
struct Ingredient {
    plant: bool,
    meat: bool,
}

#[derive(Debug, PartialEq)]
struct Meal {
    ingredients: Vec<Ingredient>,
}

#[derive(Copy, Clone)]
enum DietaryRestriction {
    Vegetarian,
    Carnivore,
    Omnivore,
}

//region Version 1: Functional, but not that functional
fn get_menu(entire_menu: &Vec<Meal>, restriction: DietaryRestriction) -> Vec<&Meal> {
    match restriction {
        Vegetarian => entire_menu
            .iter()
            .filter(|meal| meal.ingredients.iter().all(|ing| ing.plant))
            .collect(),
        Carnivore => entire_menu
            .iter()
            .filter(|meal| meal.ingredients.iter().any(|ing| ing.meat))
            .collect(),
        Omnivore => entire_menu.iter().collect(),
    }
}
//endregion

//region Version 2: Functional, but we can be even more functional
fn get_menu_2(entire_menu: &Vec<Meal>, restriction: DietaryRestriction) -> Vec<&Meal> {
    entire_menu
        .iter()
        .filter(|meal| match restriction {
            Vegetarian => meal.ingredients.iter().all(|ing| ing.plant),
            Carnivore => meal.ingredients.iter().any(|ing| ing.meat),
            Omnivore => true,
        })
        .collect()
}
//endregion

//region Version 3: Really functional, but...
fn dietary_restriction_predicate(restriction: DietaryRestriction)
    -> Box<dyn Fn(&Meal) -> bool>
{
    Box::new(
        match restriction {
            Vegetarian =>
                |meal| meal.ingredients.iter().all(|ing| ing.plant),
            Carnivore =>
                |meal| meal.ingredients.iter().any(|ing| ing.meat),
            Omnivore =>
                |meal| true,
        })
}

fn get_menu_by<Pred>(entire_menu: &Vec<Meal>, pred: Pred) -> Vec<&Meal>
    where Pred: Fn(&Meal) -> bool
{
    entire_menu
        .iter()
        .filter(|meal| pred(meal) )
        .collect()
}

fn get_menu_3(entire_menu: &Vec<Meal>, restriction: DietaryRestriction) -> Vec<&Meal> {
    get_menu_by(entire_menu, dietary_restriction_predicate(restriction))
}
//endregion

#[cfg(test)]
mod tests {
    use crate::DietaryRestriction::{Carnivore, Omnivore, Vegetarian};
    use super::*;

    #[test]
    fn test_get_menu() {

        //region entire_menu
        let entire_menu = vec![
            Meal {
                ingredients: vec![
                    Ingredient {
                        plant: true,
                        meat: false,
                    },
                    Ingredient {
                        plant: true,
                        meat: false,
                    },
                ],
            },
            Meal {
                ingredients: vec![
                    Ingredient {
                        plant: false,
                        meat: true,
                    },
                    Ingredient {
                        plant: true,
                        meat: false,
                    },
                ],
            },
            Meal {
                ingredients: vec![
                    Ingredient {
                        plant: false,
                        meat: true,
                    },
                    Ingredient {
                        plant: false,
                        meat: true,
                    },
                ],
            },
        ];
        //endregion

        let dietary_restrictions = vec![Vegetarian, Carnivore, Omnivore];
        let implementations = vec![get_menu, get_menu_2, get_menu_3];

        dietary_restrictions.iter().for_each(|restriction| {
            let mut menus: Vec<_> = implementations.iter().map(|impl_fn| {
                impl_fn(&entire_menu, *restriction)
            }).collect();
            menus.dedup();
            assert_eq!(menus.len(), 1);
        });
    }
}

fn main() {

}
