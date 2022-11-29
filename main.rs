trait Log {
    fn display_info(&self);
    // fn alert_somethig() {
    //     println!("Default implementation");
    // }
    fn alert_self(&self) {
        println!("Default");
    }
}

#[derive(Debug)]
enum PersonId {
    Passport(u32, u32, u32),
    IdentityCard(String),
}

struct Person {
    name: String,
    last_name: String,
    age: u32,
    id: PersonId,
}

struct Animal(String, u32, String);

impl Log for Animal {
    fn display_info(&self) {
        println!("{}", &self.0);
    }
    // fn alert_somethig() {
    //     println!("Impl default");
    // }
}

impl Log for Person {
    fn display_info(&self) {
        println!(
            "{} {} {} {:?}",
            self.name, self.last_name, self.age, self.id
        );
    }
}

impl Person {
    fn new() -> Person {
        Person {
            name: "Default".to_string(),
            last_name: "Default".to_string(),
            age: 0,
            id: PersonId::IdentityCard("XY1221245".to_string()),
        }
    }

    fn from(name: String, last_name: String, age: u32, id: PersonId) -> Person {
        Person {
            name,
            last_name,
            age,
            id,
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

    /* fn display_info(&self) {
        println!(
            "{} {} {} {:?}",
            self.name, self.last_name, self.age, self.id
        );
    } */
}

fn main() {
    Person::some_function();

    let person = Person {
        name: "Nick".to_string(),
        last_name: "Kim".to_string(),
        age: 31,
        id: PersonId::IdentityCard("EW1241241".to_string()),
    };

    let person_2 = Person::new();
    let mut person_3 = Person::from(
        String::from("Nick2"),
        String::from("Kim"),
        6,
        PersonId::Passport(123, 124, 233),
    );

    person_2.display_age();

    person_3.change_age(7);

    println!(
        "{} {} {} {:?}",
        person.name, person.last_name, person.age, person.id
    );
    person_3.display_info();

    check_persion_id(person.id);

    // Animal::alert_somethig();
    // Person::alert_somethig();

    person_2.alert_self();

    log_info(person_2);
    log_info_2(&person_3);
}

fn check_persion_id(id: PersonId) {
    if let PersonId::Passport(x, y, z) = id {
        println!("It matching Passport {} {} {}", x, y, z);
    } else {
        println!("It doesn't match!");
    }

    match id {
        PersonId::Passport(x, _, _) => {
            println!("Passport: first value - {}", x);
        }
        PersonId::IdentityCard(x) => {
            println!("ID Card - {}", x);
        }
    }

    let animal = Animal("dog".to_string(), 10, "bulldog".to_string());

    animal.display_info();

    let Animal(animal_type, _, _) = animal;
    println!("{}", animal_type);
}

// impl makes the compiler determine type at the compile time
// it will create multiple version of the function, depending on
// how many types Log trait implements (Person, Animal)
fn log_info(val: impl Log) {
    val.alert_self();
}
// fn log_info_asdgerq(val: Person) {
//     val.alert_self();
// }
// fn log_info_gdasg(val: Animal) {
//     val.alert_self();
// }

// dession of exactly which function to call at the runtime
fn log_info_2(val: &dyn Log) {
    val.alert_self();
}
