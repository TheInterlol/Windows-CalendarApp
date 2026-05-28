pub enum TypesOfDay {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl TypesOfDay {
    pub fn get_number_of_day(&self) -> u8 {
        match self {
            TypesOfDay::Monday => 1,
            TypesOfDay::Tuesday => 2,
            TypesOfDay::Wednesday => 3,
            TypesOfDay::Thursday => 4,
            TypesOfDay::Friday => 5,
            TypesOfDay::Saturday => 6,
            TypesOfDay::Sunday => 7,
        }
    }
}
