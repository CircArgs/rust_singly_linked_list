mod linked_list;
use linked_list::List;

fn main() {
    let mut list = List::new();
    list.append(1);
    list.append(2);
    list.append(4);
    list.append(5);
    for i in &list {
        println!("{:?}", i);
    }
    println!("{:?}", list);
}
