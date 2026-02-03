use chrono::prelude::*;
// alternativamente pode ser usado 'Datelike'

fn main() {
    let function_ptr: fn() = print_today_date;
    run_function(function_ptr);

}

fn run_function(func_ptr: fn()){
    func_ptr()
}

fn print_today_date(){
    let t = Local::now();
    let (year, month, day) = (t.year(), t.month(), t.day());
    println!("Hoje é dia {}/{}/{}.", day, month, year);

}