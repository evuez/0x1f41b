use std::vec::Vec;
use parser::{Element, Expression, Operator};

pub fn test(elements: Vec<Element>) {
    let r0 = filter(&elements, &|e| { match e {
        &Element::Expression(Expression {operator: Operator::Store, ..}) => true,
        _ => false
    } });
    let r1 = filter_and_extract(&elements, &|e| { match e {
        &Element::Expression(Expression {operator: Operator::Store, ..}) => true,
        _ => false
    } }, &|e| { match e {
        &Element::Expression(Expression {elements: elements, ..}) => Element::Value(1),
        _ => Element::Value(1)
    } });

    println!("{:?}", r0);
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
    elements.iter().filter(|e| matcher(e)).collect::<Vec<&Element>>()
}

fn filter_and_extract<'a, F, G>(elements: &'a Vec<Element>, matcher: &F, extractor: &G) -> Vec<Element>
        where F: Fn(&Element) -> bool, G: Fn(&Element) -> Element {
    elements.iter().filter(|e| matcher(e)).map(|e| extractor(e)).collect::<Vec<Element>>()
}
