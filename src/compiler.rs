use std::vec::Vec;
use parser::{Element, Expression, Operator};

pub fn test(elements: Vec<Element>) {
    let r = filter(&elements, &|e| { match e {
        &Element::Expression(Expression {operator: Operator::Store, ..}) => true,
        _ => false
    } });
    println!("{:?}", r);
}

fn substitute(elements: Vec<Element>) {
}

// fn find_element<F>(elements: Vec<Element>, matcher: &F) -> Vec<Expression> where F: Fn(Operator) -> bool {
//    for element in elements {
//        match element {
//            Element::Expression(e) => walk_expression(e, &matcher),
//            _ => ()
//        }
//    }
// }
//
// fn walk_expression<F>(expression: Expression, matcher: &F) -> Vec<Expression> where F: Fn(Operator) -> bool {
//     match expression {
//         Expression { operator: Operator::Store, .. } =>
//     }
// }


fn filter<'a, F>(elements: &'a Vec<Element>, matcher: &F) -> Vec<&'a Element> where F: Fn(&Element) -> bool {
    //(1..101).filter(|&v| matcher(v as i32)).collect::<Vec<i32>>()
    elements
        .iter()
        .filter(|e| matcher(e))
        .collect::<Vec<&Element>>()
}

