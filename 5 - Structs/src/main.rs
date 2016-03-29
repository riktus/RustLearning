struct human{
    name: String,
    age: i32,
}

fn main(){
    let man = human{
        name: "riktus".to_string(),
        age: 25
    };

    println!("");
    println!("{} is {} age old", man.name, man.age);

    change_age(26, man);
}

fn change_age(age: i32, mut target: human){
    target.age = age;

    println!("");
    println!("Age was changed");
    println!("");
    println!("{} is {} age old", target.name, target.age);
}
