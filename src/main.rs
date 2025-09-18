mod memtable;

use memtable::Memtable;

fn main() {
    println!("Hello, world!");
    let mut mem = Memtable::new();
    mem.put(b"test".to_vec(), b"demo".to_vec());

    if let Some(val) = mem.get(b"test".to_vec()) {
        println!("Found {:?}", String::from_utf8(val));
    }
}
