use chrono::prelude::*;
// alternativamente pode ser usado 'Datelike'

fn main() {
    print_today_date();
}

fn print_today_date(){
    let t = Local::now();
    let (year, month, day) = (t.year(), t.month(), t.day());
    println!("Hoje é dia {}/{}/{}.", day, month, year);

}