use linked_list::LinkedList;

mod linked_list;

fn main() {
    let mut list = LinkedList::<u32>::new();
    for i in 0..10 {
        list.add(i);
    }

    list.insert(11, 2);
    println!("size: {} contetns: {}", &list.len(),  &list);
}
