// Атрибут, который убирает предупреждения компилятора
// о неиспользуемом коде
#![allow(dead_code)]

// Создадим `enum`, для классификации кого либо. Обратите внимание на то, как
// and type information together specify the variant:
// `Engineer != Scientist` and `Height(i32) != Weight(i32)`. Each
// is different and independent.
enum Person {
    // `enum` так же может быть `единичным`,
    Engineer,
    Scientist,
    // может быть как кортежная структура,
    Height(i32),
    Weight(i32),
    // или как просто структура.
    Info { name: String, height: i32 }
}

// Функция, которая принимает Person` в качестве аргумента
// и не возвращает ничего.
fn inspect(p: Person) {
    // При использование `enum` компилятору необходимо указать, 
    // что нужно делать при каждом из возможных вариантов.
    // Для этого мы используем `match`.
    match p {
        Person::Engineer  => println!("Is an engineer!"),
        Person::Scientist => println!("Is a scientist!"),
        // Деструктурируем `i` из `enum`.
        Person::Height(i) => println!("Has a height of {}.", i),
        Person::Weight(i) => println!("Has a weight of {}.", i),
        // Деструктурируем `Info` в `name` и `height`.
        Person::Info { name, height } => {
            println!("{} is {} tall!", name, height);
        },
    }
}

fn main() {
    let person   = Person::Height(18);
    let amira    = Person::Weight(10);
    // `to_owned()` создает копию `String` из среза строки.
    let dave     = Person::Info { name: "Dave".to_owned(), height: 72 };
    let rebecca  = Person::Scientist;
    let rohan    = Person::Engineer;

    inspect(person);
    inspect(amira);
    inspect(dave);
    inspect(rebecca);
    inspect(rohan);
}
