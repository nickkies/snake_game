struct Person {
    name: String,
    last_name: String,
    age: u32,
}

impl Person {
    fn new() -> Person {
        Person {
            name: "Default".to_string(),
            last_name: "Default".to_string(),
            age: 0,
        }
    }

    fn from(name: String, last_name: String, age: u32) -> Person {
        Person {
            name,
            last_name,
            age,
        }
    }

    // associated function
    fn some_function() {
        println!("some_function");
    }

    // method
    // first parameter is always self, which repersents the instance of the struct the
    // method is being called on
    // Within an impl block, the type Self is an alias for the current type
    fn display_age(&self) {
        println!("Current Age: {}", self.age);
    }

    fn change_age(&mut self, new_age: u32) {
        self.age = new_age;
    }
}

fn main() {
    Person::some_function();

    let person = Person {
        name: "Nick".to_string(),
        last_name: "Kim".to_string(),
        age: 31,
    };

    let person_2 = Person::new();
    let mut person_3 = Person::from(String::from("Nick2"), String::from("Kim"), 6);

    person_2.display_age();

    person_3.change_age(7);

    println!("{}, {} , {}", person.name, person.last_name, person.age);
}
