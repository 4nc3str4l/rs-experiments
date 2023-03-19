use linked_list::LinkedList;

mod linked_list;

fn main() {
    let mut list = LinkedList::<u32>::new();
    for i in 0..10 {
        list.add(i);
    }

    list.insert(11, 2);
    println!("size: {} contetns: {}", &list.len(),  &list);
    list.remove(11);
    println!("size: {} contetns: {}", &list.len(),  &list);
    list.remove(0);
    println!("size: {} contetns: {}", &list.len(),  &list);
    list.remove(9);
    println!("size: {} contetns: {}", &list.len(),  &list);

    for i in 0..20 {
        if i != 4 {
            list.remove(i);
        }
        println!("size: {} contetns: {}", &list.len(),  &list);
    }

    list.remove(4);
    println!("size: {} contetns: {}", &list.len(),  &list);
    
}
