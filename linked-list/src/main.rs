use linked_list::LinkedList;

mod linked_list;

fn main() {
    let mut list = LinkedList::<u32>::new();
    for i in 0..10 {
        list.push(i);
    }
    println!("size: {} contetns: {}", &list.len(),  &list);
}
