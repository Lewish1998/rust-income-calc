slint::include_modules!();
use chrono::prelude::*;

const BILLS: f64 = 0.555;
const FOOD: f64 = 0.083;
const TRAVEL: f64 = 0.055;
const ENTERTAINMENT: f64 = 0.027;
const SAVINGS: f64 = 0.166;
const MISC: f64 = 0.114;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let ui_handle = ui.as_weak();
    ui.on_divide_income(move |string| {
        let local: DateTime<Local> = Local::now();
        let year: i32 = local.year();
        let month: u32 = local.month();
        let day: u32 = local.day();
        let date: String = format!("{}-{}-{}", year, month, day);

        let ui = ui_handle.unwrap();
        let num: f64 = string.trim().parse().unwrap();
        let bills: f64 = num * BILLS;
        let food: f64 = num * FOOD;
        let travel: f64 = num * TRAVEL;
        let entertainment: f64 = num * ENTERTAINMENT;
        let savings: f64 = num * SAVINGS;
        let misc: f64 = num * MISC;
        let remainder: f64 = num - (bills + food + travel + entertainment + savings + misc);
        let result: String = format!(
            "Bills: {:.2}\nFood: {:.2}\nTravel: {:.2}\nEntertainment: {:.2}\nSavings: {:.2}\nMisc: {:.2} \nRemainder: {:.2}",
            bills, food, travel, entertainment, savings, misc, remainder
        );
        let transfer: String = format!(
            "Move {:.2} to Bills Account\nMove {:.2} to Savings Account\nKeep {:.2} in Main Account",
            (bills+food), (savings+remainder), (entertainment+misc+travel)
        );
        let yearsaving: String = format!(
            "You could save {:.2} in a year", (savings+remainder)*12.0
        );
        ui.set_local(date.into());
        ui.set_results(result.into());
        ui.set_transfer(transfer.into());
        ui.set_savings(yearsaving.into());
    });

    ui.run()
}
