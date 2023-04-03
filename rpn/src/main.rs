mod exit;

use exit::exit;
use std::env;
use std::process::exit;

#[derive(Debug)]
enum Operator {
    MULTI,
    DIV,
    SUBST,
    ADD,
}

#[derive(Debug)]
enum Item {
    Number(i64),
    Operator(Operator),
}

impl Item {
    fn is_number(&self) -> bool {
        match self {
            Item::Number(_) => true,
            _ => false,
        }
    }

    fn is_operator(&self) -> bool {
        match self {
            Item::Operator(_) => true,
            _ => false,
        }
    }

    fn get_nb(&self) -> Option<&i64> {
        match self {
            Item::Number(nb) => Some(nb),
            _ => None,
        }
    }

    fn get_op(&self) -> Option<&Operator> {
        match self {
            Item::Operator(op) => Some(op),
            _ => None,
        }
    }
}

fn validate_expression(elements: &Vec<Item>) -> bool {
    if elements.len() < 3 {
        return false;
    }

    let operators_count = elements.iter().filter(|item| item.is_operator()).count();
    let numbers_count = elements.iter().filter(|item| item.is_number()).count();

    return elements[0].is_number()
        && elements[1].is_number()
        && numbers_count - 1 == operators_count;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        exit!("Error", 1)
    }
    let expression = args.get(1).expect("args len is 2");

    let mut elements: Vec<Item> = expression
        .split(" ")
        .map(|item| {
            let item = item.trim();
            let nb = item.parse::<i64>();
            if nb.is_ok() {
                return Item::Number(nb.expect("nb is ok"));
            }
            match item {
                "*" => Item::Operator(Operator::MULTI),
                "/" => Item::Operator(Operator::DIV),
                "+" => Item::Operator(Operator::ADD),
                "-" => Item::Operator(Operator::SUBST),
                _ => exit!("Error", 1),
            }
        })
        .collect();

    if !validate_expression(&elements) {
        exit!("Error", 1)
    }

    while let Some(operator_index) = elements.iter().position(|el| el.is_operator()) {
        let right_index = operator_index - 1;
        let left_index = operator_index - 2;

        let Some(right_nb) = elements .get(right_index) else {
            exit!("Error", 1);
        };
        let Some(right_nb) = right_nb.get_nb().copied() else {
            exit!("Error", 1);
        };
        let Some(left_nb) = elements.get(left_index) else {
            exit!("Error", 1)
        };
        let Some(left_nb) = left_nb.get_nb().copied() else {
            exit!("Error", 1)
        };

        let result = match elements
            .get(operator_index)
            .expect("operator index should be valid")
            .get_op()
            .expect("item should be an operator variant")
        {
            Operator::MULTI => Some(left_nb * right_nb),
            Operator::SUBST => Some(left_nb - right_nb),
            Operator::ADD => Some(left_nb + right_nb),
            Operator::DIV => left_nb.checked_div(right_nb),
        };

        if result.is_none() {
            exit!("Error", 1)
        }

        elements[left_index] = Item::Number(result.expect("result is not none"));
        elements.remove(operator_index);
        elements.remove(right_index);
    }

    println!("{:?}", elements.get(0).unwrap().get_nb().unwrap());
}
