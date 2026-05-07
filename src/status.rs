use std::{thread::sleep, time::Duration};

use crate::{PAUSE_TIME, cards::advice::AdviceType};

#[derive(Debug, PartialEq)]
pub(crate) enum StatusType {
    // Player will miss their next go(s)
    MissGo(u8),
    // ??
    NoCountries(u8),
    // Affected by Bad Advice
    BadAdvice(AdviceType),
    // Affected by Visa Problem
    VisaProblem,
}

#[derive(Debug)]
pub(crate) struct StatusHandler {
    types: Vec<StatusType>,
}

impl StatusHandler {
    pub(crate) fn empty() -> Self {
        Self { types: Vec::new() }
    }

    pub(crate) fn add_status(&mut self, status: StatusType) {
        self.types.push(status);
    }

    fn remove_status(&mut self, status: StatusType) {
        self.types.retain(|t| *t != status);
    }

    pub(crate) fn no_turn(&mut self) -> bool {
        for ty in self.types.iter_mut() {
            if let StatusType::MissGo(gos) = ty {
                if gos > &mut 1 {
                    println!("Missing this go. {} more to go", gos);
                } else {
                    println!("Missing this go.");
                }
                *gos -= 1;

                self.cleanup();

                sleep(Duration::from_millis(PAUSE_TIME));
                return true;
            }
        }

        // sleep(Duration::from_millis(PAUSE_TIME));
        false
    }

    fn cleanup(&mut self) {
        self.types.retain(|ty| match ty {
            StatusType::MissGo(gos) => *gos != 0,
            _ => true,
        });
    }
}
