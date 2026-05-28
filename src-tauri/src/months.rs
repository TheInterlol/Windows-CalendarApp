pub enum TypesOfMonth {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

impl TypesOfMonth {
    pub fn get_days_in_month(&self, year: i32) -> u8 {
        match self {
            TypesOfMonth::January => 31,
            TypesOfMonth::February => {
                if year % 4 == 0 {
                    29
                } else {
                    28
                }
            }
            TypesOfMonth::March => 31,
            TypesOfMonth::April => 30,
            TypesOfMonth::May => 31,
            TypesOfMonth::June => 30,
            TypesOfMonth::July => 31,
            TypesOfMonth::August => 31,
            TypesOfMonth::September => 30,
            TypesOfMonth::October => 31,
            TypesOfMonth::November => 30,
            TypesOfMonth::December => 31,
        }
    }
}
