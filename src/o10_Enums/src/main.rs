// -------------------------------------------
// 	Enums
// -------------------------------------------
//

enum Weekdays {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

impl Weekdays{
    fn day_name(&self) -> &str {
        let day = match self {
            Weekdays::Friday => "Friday",
            Weekdays::Monday => "Monday",
            Weekdays::Saturday => "Saturday",
            Weekdays::Sunday => "Sunday",
            Weekdays::Thursday => "Thursday",
            Weekdays::Tuesday => "Tuesday",
            Weekdays::Wednesday => "Wednesday",
        };
        day
    }
}

fn main() {

    let day = Weekdays::Friday;
    print!("Today: {}", day.day_name());
}
