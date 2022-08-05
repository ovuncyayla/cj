use cj::util::Map;


fn main() {
        
    let mut map = Map::<&str, &str>::new();

    map.insert("1", "1");
    map.insert("2", "2");
    map.insert("3", "3");
    map.insert("4", "4");
    map.insert("5", "5");
    map.insert("6", "6");


    println!("{:?}", map);
    map.remove("3");
    println!("{:?}", map);
    let a = map.get(&"6");

    println!("{:?}", a);
}