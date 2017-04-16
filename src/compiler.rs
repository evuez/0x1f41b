use std::vec::Vec;
use parser::{Element, Expression, Operator};

pub fn test(elements: Vec<Element>) {
    let r = find_expression(elements, &|o| { match o { Operator::Store => true, _ => false } });
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
