use std::collections::LinkedList;

fn main() {
    let mut list = LinkedList::new();
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    list.push_back(5);
    list.push_back(4);

    for element in list.iter() {
        println!("{}", element);
    }
}
