mod another_lib;

use another_lib::another_mod;

fn outsider() {
    another_mod::another_fn();
    println!("outsider fn");
}

pub mod learning_rust {
    use std::fmt;

    mod top_level {
        pub fn hi_there() {
            println!("hi there");
        }

        pub mod low_level {
            pub fn hello_world() {
                println!("hello_world");
            }
        }
    }
    pub trait Log {
        fn display_info(&self);
        // fn alert_somethig() {
        //     println!("Default implementation");
        // }
        fn alert_self(&self) {
            println!("Default");
        }
    }

    pub enum PersonId {
        Passport(u32, u32, u32),
        IdentityCard(String),
    }

    impl fmt::Display for PersonId {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                PersonId::IdentityCard(x) => {
                    write!(f, "IdentityCard: {}", x)
                }
                PersonId::Passport(x, y, z) => {
                    write!(f, "Passport: {} {} {}", x, y, z)
                }
            }
        }
    }

    pub struct Person {
        pub name: String,
        pub last_name: String,
        pub age: u32,
        pub id: PersonId,
    }

    pub struct Animal(pub String, pub u32, pub String);

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
            super::outsider();
            crate::outsider();

            // crate points to -> src/lib.rs and src/main.rs
            crate::learning_rust::top_level::hi_there();
            crate::learning_rust::top_level::low_level::hello_world();

            top_level::hi_there();
            top_level::low_level::hello_world();

            println!("{} {} {} {}", self.name, self.last_name, self.age, self.id);
        }
    }

    impl Person {
        pub fn new() -> Person {
            Person {
                name: "Default".to_string(),
                last_name: "Default".to_string(),
                age: 0,
                id: PersonId::IdentityCard("XY1221245".to_string()),
            }
        }

        pub fn from(name: String, last_name: String, age: u32, id: PersonId) -> Person {
            Person {
                name,
                last_name,
                age,
                id,
            }
        }

        // associated function
        pub fn some_function() {
            println!("some_function");
        }

        // method
        // first parameter is always self, which repersents the instance of the struct the
        // method is being called on
        // Within an impl block, the type Self is an alias for the current type
        pub fn display_age(&self) {
            println!("Current Age: {}", self.age);
        }

        pub fn change_age(&mut self, new_age: u32) {
            self.age = new_age;
        }

        pub fn name(&self) -> &String {
            &self.name
        }

        /* fn display_info(&self) {
            println!(
                "{} {} {} {:?}",
                self.name, self.last_name, self.age, self.id
            );
        } */
    }

    // impl makes the compiler determine type at the compile time
    // it will create multiple version of the function, depending on
    // how many types Log trait implements (Person, Animal)
    pub fn log_info(val: impl Log) {
        val.alert_self();
    }
    // fn log_info_asdgerq(val: Person) {
    //     val.alert_self();
    // }
    // fn log_info_gdasg(val: Animal) {
    //     val.alert_self();
    // }

    // dession of exactly which function to call at the runtime
    pub fn log_info_2(val: &dyn Log) {
        val.alert_self();
    }
}
