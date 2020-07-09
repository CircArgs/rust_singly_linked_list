mod first;
use first::List;

fn main() {
    let mut list = List::new();
    list.append(1);
    list.append(2);

    println!("{:?}", list);
}
