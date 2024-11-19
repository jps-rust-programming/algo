use std::collections::HashMap;
pub fn create_hashmap() {
    let mut map = HashMap::new();
    map.insert("key1".to_string(), 10);
    map.insert("key2".to_string(), 20);
    map.insert("key3".to_string(), 30);
    map.insert("key1".to_string(), 40);
    map.insert("key1".to_string(), 50);
    map.insert("key1".to_string(), 60);
    println!("Hash map: {:?}", map);
    if let Some(rm) = remove_value(&mut map, "key1") {
        println!("Removed value: {}", rm);
    }
    println!("Hash map: {:?}", map);
    if find_key(&mut map, "key1") {
        println!("key exist!")
    }
    looping_element(&mut map);
}

pub fn get_value() {
    let mut map = HashMap::new();
    map.insert("key1", 10);
    map.insert("key2", 200);

    if let Some(value) = map.get("key1") {
        println!("Value for key1: {}", value);
    } else {
        println!("key not found");
    }
}

// pub fn remove_element<'a>(map: &'a mut HashMap<String, i32>, key: &'a str) -> Option<&'a i32>{
//     map.remove(key)
// }

pub fn remove_value(map: &mut HashMap<String, i32>, key: &str) -> Option<i32> {
    map.remove(key)
}

pub fn find_key(map: &mut HashMap<String, i32>, key: &str) -> bool {
    map.contains_key(key)
}

pub fn looping_element(map: &mut HashMap<String, i32>) {
    *map.entry("key4".to_string()).or_insert(100);

    for (key, value) in map {
        println!("key: {} value: {}", key, value);
    }
    println!("Length: {}", map.len());
    println!("Capacity: {}", map.capacity());
}
