extern crate custom_error;
use custom_error::custom_error;
use holidays::{Country, Holiday};

custom_error! {pub HolidayParseError
    UnknownCountryCode { code: String } = "unknown country code {code}",
    UnknownHolidayName { name: String, names: String } = "`{name}` not found in {names}"
}

pub fn country_from_code(code: String) -> Result<holidays::Country, HolidayParseError> {
    match code.as_str() {
        "US" => Ok(holidays::Country::US),
        _ => Err(HolidayParseError::UnknownCountryCode { code }),
    }
}

pub fn holidays(country: Country, year: holidays::Year) -> Vec<Holiday> {
    let holidays = holidays::Builder::new()
        .countries(&[country])
        .years(year..year + 1)
        .build()
        .unwrap();
    let all_holidays = holidays.get(&country).unwrap().get(&year).unwrap();
    all_holidays.to_owned().into_values().collect()
}

pub fn paid_holidays_from_names(
    paid_holiday_names: Vec<String>,
    country: Country,
    year: holidays::Year,
) -> Result<Vec<Holiday>, HolidayParseError> {
    let all_holidays = holidays(country, year);

    let mut paid_holidays: Vec<Holiday> = vec![];

    let mut nonobserved_dates = vec![];
    for holiday in &all_holidays {
        if !holiday.name.contains("(Observed)") {
            for observed_holiday in &all_holidays {
                if observed_holiday.name.contains(holiday.name.as_str())
                    && observed_holiday.name.contains("(Observed)")
                {
                    nonobserved_dates.push(holiday.date);
                    break;
                }
            }
        }
    }

    let holidays: Vec<Holiday> = all_holidays
        .into_iter()
        .filter(|holiday| !nonobserved_dates.contains(&holiday.date))
        .map(|holiday| holiday.to_owned())
        .collect();

    for paid_holiday_name in &paid_holiday_names {
        let mut found = false;
        for holiday in &holidays {
            if holiday.name.starts_with(paid_holiday_name) {
                paid_holidays.push(holiday.to_owned());
                found = true;
                break;
            }
        }
        if !found {
            let holiday_names: Vec<String> = holidays
                .iter()
                .map(|holiday| holiday.name.to_owned())
                .collect();
            return Err(HolidayParseError::UnknownHolidayName {
                name: paid_holiday_name.to_string(),
                names: format!("{:?}", holiday_names),
            });
        }
    }

    Ok(paid_holidays)
}
