use std::sync::{Mutex, Once};
use std::mem::MaybeUninit;
use std::collections::HashSet;
use rand::Rng;


type Registry = HashSet<String>;

fn get_registry() -> &'static Mutex<Registry> {
    static mut REGISTRY: MaybeUninit<Mutex<Registry>> = MaybeUninit::uninit();
    static REGISTRY_ONCE: Once = Once::new();
    unsafe {
        REGISTRY_ONCE.call_once(|| {
            REGISTRY.write(Mutex::new(Registry::new()));
        });
        REGISTRY.assume_init_ref()
    }
}

fn new_name(registry: &mut Registry) -> String {
    let mut rng = rand::thread_rng();
    loop {
        let letter_first: char = rng.gen_range(b'A'..b'Z') as char;
        let letter_second: char = rng.gen_range(b'A'..b'Z') as char;
        let number = rng.gen_range(0..999);
        let mut name = String::from("");
        name.push(letter_first);
        name.push(letter_second);
        let candidate = name + &format!("{:03}", number);
        if !registry.contains(&candidate) {
            registry.insert(candidate.clone());
            return candidate;
        }
    }
}


pub struct Robot {
    name: String,
}

impl Robot {
    pub fn new() -> Self {
        let mut registry = get_registry().lock().unwrap();
        let name = new_name(&mut registry);
        Robot { name }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset_name(&mut self) {
        let mut registry = get_registry().lock().unwrap();
        let new_name = new_name(&mut registry);
        registry.remove(&self.name);
        self.name = new_name;
    }
}
