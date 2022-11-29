use snake_game::learning_rust::{log_info, log_info_2, Animal, Log, Person, PersonId};

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
        "{} {} {} {}",
        person.name(),
        person.last_name,
        person.age,
        person.id
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
