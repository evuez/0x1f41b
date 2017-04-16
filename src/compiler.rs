use std::vec::Vec;
use parser::Element;

pub fn test(elements: Vec<Element>) {
    let r = find_stores(elements);
    println!("{:?}", r);
}

fn substitute(elements: Vec<Element>) {
}

fn find_stores(elements: Vec<Element>) {
    let r = elements
        .iter()
        .filter(|&&el| match el {
            Element::Expression(e) => false,
            _ => true
        })
        .collect::<Vec<_>>();
    println!("{:?}", r);
}

