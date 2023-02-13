enum Sex {
    Male,
    Female
}

enum Position {
    Intern,
    Junior,
    Senior
}

struct User {
    name: String,
    age: u64,
    sex: Sex,
    position: Position
}

let _name = String::from("Berkin");
let _age = 17;
let _sex = Sex::Male;
let _position = Position::Junior;

let mut user = User {
    name: _name,
    age: _age,
    sex: _sex,
    position: _position
};

impl User {
    fn is_egligible(&self) -> bool {
        match self.position {
            Position::Intern => false,
            Position::Junior => false,
            Position::Senior => true
        }
    }

}