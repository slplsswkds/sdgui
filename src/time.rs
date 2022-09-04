use std::cell::RefCell;
use std::rc::Rc;

use std::string::String;

use chrono;

#[derive(Clone)]
pub struct Time24 {
    pub hours: Rc<RefCell<i32>>,
    pub minutes: Rc<RefCell<i32>>,
}

impl Time24 {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            hours: Rc::from(RefCell::from(0)),
            minutes: Rc::from(RefCell::from(0)),
        }
    }

    #[allow(dead_code)]
    pub fn now(&mut self) {
        let time = chrono::offset::Local::now().time();
        let hours: i32 = time.format("%H").to_string().parse().unwrap();
        let minutes: i32 = time.format("%M").to_string().parse().unwrap();

        *self.hours.borrow_mut() = hours;
        *self.minutes.borrow_mut() = minutes;
    }

    #[allow(dead_code)]
    pub fn to_str(&self) -> String {
        let hours = *self.hours.borrow();
        let mut hours_str = hours.to_string();
        if hours < 10 {
            hours_str = String::from("0") + &hours_str;
        }

        let minutes = *self.minutes.borrow();
        let mut minutes_str = minutes.to_string();
        if minutes < 10 {
            minutes_str = String::from("0") + &minutes_str;
        }

        let time_str = hours_str + ":" + &minutes_str;
        return time_str;
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        println!("{}", self.to_str());
    }

    #[allow(dead_code)]
    pub fn add_hours(&mut self, hours_to_add: i32) {
        let mut hours = *self.hours.borrow();
        hours += hours_to_add;
        if hours > 23 {
            hours -= 24;
        }
        *self.hours.borrow_mut() = hours;
    }

    #[allow(dead_code)]
    pub fn add_minutes(&mut self, minutes_to_add: i32) {
        let mut minutes = *self.minutes.borrow();
        minutes += minutes_to_add;
        if minutes > 59 {
            minutes = 0;
            self.add_hours(1);
        }
        *self.minutes.borrow_mut() = minutes;
    }

    #[allow(dead_code)]
    pub fn subtract_hours(&mut self, hours_to_subtract: i32) {
        let mut hours = *self.hours.borrow();
        hours -= hours_to_subtract;
        if hours < 0 {
            hours += 24;
        }
        *self.hours.borrow_mut() = hours;
    }

    #[allow(dead_code)]
    pub fn subtract_minutes(&mut self, minutes_to_subtract: i32) {
        let mut minutes = *self.minutes.borrow();
        minutes -= minutes_to_subtract;
        if minutes < 0 {
            minutes += 60;
            self.subtract_hours(1);
        }
        *self.minutes.borrow_mut() = minutes;
    }

    pub fn eq_zero(&self) -> bool {
        if *self.hours.borrow() == 0 && *self.minutes.borrow() == 0 {
            true
        } else {
            false
        }
    }

    pub fn h(&self) -> i32 {
        return *self.hours.borrow();
    }

    pub fn m(&self) -> i32 {
        return *self.minutes.borrow();
    }

    pub fn eq(&self, time: &Self) -> bool {
        if self.h() == time.h() && self.m() == time.m() {
            return true;
        }
        return false;
    }
}
