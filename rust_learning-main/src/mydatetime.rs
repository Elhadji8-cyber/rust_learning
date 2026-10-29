extern crate chrono;
use std::time::{Duration, Instant};

use chrono::NaiveDate;


pub fn test_stdtime(){
    let dur1 = Duration::from_secs(15); 
    println!("{}", dur1.as_millis());

    let dur2 = Duration::from_millis(5500);
    let dur3 = dur1.checked_sub(dur2);
    println!("The difference between the two durations is: {}", dur3.unwrap_or_default().as_millis());

    let now = Instant::now();
    std::thread::sleep(Duration::from_millis(200));     
    println!("Elapsed time is: {} milliseconds", now.elapsed().as_millis());                                                                                                                                                                                                                                                                                                            
}


pub fn test_chrono(){

    let utc_now =  chrono::Utc::now();
    print!("{}", utc_now.format(" %Y %b %d %H %M %S.%f %Z\n"));

    let local_time = chrono::Local::now();
    
    let date1  = NaiveDate::from_isoywd_opt(2024, 1, chrono::Weekday::Mon);
    let unwrapped_date = date1.unwrap();
    println!("Unwrapped date is: {}", unwrapped_date.format(" %Y %b %d"));
    println!("{}", local_time.format(" %Y %b %d %H %M %S.%f %Z\n"));
}































