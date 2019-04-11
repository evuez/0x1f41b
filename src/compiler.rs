use std::vec::Vec;
use parser::{Element, Expression, Operator};

pub fn test(elements: Vec<Element>) {
//    let r0 = filter(&elements, &|e| { match e {
//        &Element::Expression(Expression {operator: Operator::Store, ..}) => true,
//        _ => false
//    } });
//    let r1 = filter_and_extract(&elements, &|e| { match e {
//        &Element::Expression(Expression {operator: Operator::Store, ..}) => true,
//        _ => false
//    } }, &|e| { match e {
//        &Element::Expression(Expression {elements: _, ..}) => Element::Value(1),
//        _ => Element::Value(1)
//    } });

    let stores = filter(&elements, &|e| { match e {
        &Element::Expression(Expression {operator: Operator::Store, ..}) => true,
        _ => false
    } }, &|e| { match e {
        &Element::Expression(Expression {operator: Operator::Store, elements: _, ..}) => (Element::Value(1), )
        _ => Element::Value(1)
    } });

    println!("{:?}", r0);
}

fn substitute(elements: Vec<Element>) {
}

fn filter<'a, F>(elements: &'a Vec<Element>, matcher: &F) -> Vec<&'a Element> where F: Fn(&Element) -> bool {
    elements.iter().filter(|e| matcher(e)).collect::<Vec<&Element>>()
}

fn filter_and_extract<'a, F, G>(elements: &'a Vec<Element>, matcher: &F, extractor: &G) -> Vec<Element>
        where F: Fn(&Element) -> bool, G: Fn(&Element) -> Element {
    elements.iter().filter(|e| matcher(e)).map(|e| extractor(e)).collect::<Vec<Element>>()
}
