use std::fmt::Display;

use colored::{Colorize, CustomColor};

use crate::{Advice, AdviceType, Board, Bonus, Card, Continent, Country, Player};

impl Display for Country {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.continent() {
            Continent::Africa =>     write!(f, "{}", self.name.custom_color(CustomColor::new(134, 80, 29))),
            Continent::Asia =>       write!(f, "{}", self.name.custom_color(CustomColor::new(196, 181, 61))),
            Continent::America =>    write!(f, "{}", self.name.custom_color(CustomColor::new(234, 83, 119))),
            Continent::Antarctica => write!(f, "{}", self.name.custom_color(CustomColor::new(220, 220, 220))),
            Continent::Europe =>     write!(f, "{}", self.name.custom_color(CustomColor::new(118, 72, 141))),
            Continent::Oceania =>    write!(f, "{}", self.name.custom_color(CustomColor::new(113, 209, 164))),
        }
    }
}


impl Display for Bonus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let color = CustomColor::new(106, 229, 218);

        match self {
            Bonus::Beach =>    write!(f, "{}", "Beach Bonus".custom_color(color)),
            Bonus::Culture =>  write!(f, "{}", "Culture Bonus".custom_color(color)),
            Bonus::Trekking => write!(f, "{}", "Trekking Bonus".custom_color(color)),
            Bonus::Wildlife => write!(f, "{}", "Wildlife Bonus".custom_color(color)),
        }
    }
}

impl Display for Advice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.variant == AdviceType::Money {
            return write!(f, "{}", "Money Talks".green())
        }

        let pre = if self.good { "Good" } else { "Bad" };
        let (c1, c2) = if self.good {
            (CustomColor::new(12, 186, 74), CustomColor::new(81, 255, 143))
        } else {
            (CustomColor::new(248, 30, 88), CustomColor::new(248, 73, 119))
        };

        match self.variant {
            AdviceType::Bureaucracy => write!(f, "{} {}", pre.custom_color(c1), "Bureaucracy".custom_color(c2)),
            AdviceType::Timing =>      write!(f, "{} {}", pre.custom_color(c1), "Timing".custom_color(c2)),
            AdviceType::Transport =>   write!(f, "{} {}", pre.custom_color(c1), "Transport".custom_color(c2)),
            _ => panic!("Wont happen"),
        }
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Card::Country(country) => country.fmt(f),
            Card::Bonus(bonus) => bonus.fmt(f),
            Card::Advice(advice) => advice.fmt(f),
            Card::Special(..) => write!(f, "soem special gy"),
        }
    }
}


impl Player {
    pub fn show_pile(&self) {
        print!("[");
        for (i, c) in self.pile.iter().enumerate() {
            if i != 0 {
                print!(", ");
            }
            print!("{}", c);
        }
        println!("]");
    }

    pub fn try_playing_all_counties(&mut self) {
        for i in 0..self.hand.len() {
            let _ = self.play_country(i);
            self.try_playing_all_bonuses();
        }
    }

    pub fn try_playing_all_bonuses(&mut self) {
        for i in 0..self.hand.len() {
            let _ = self.play_bonus(i);
        }
    }
}



impl Card {
    pub fn deck() -> Vec<Self> {
        vec![
            // Africa
            Card::Country(Country::new("Mali",         10, "cw")),
            Card::Country(Country::new("Egypt",        10, "cw")),
            Card::Country(Country::new("Kenya",        8,  "bcw")),
            Card::Country(Country::new("Morocco",      6,  "ct")),
            Card::Country(Country::new("Uganda",       6,  "ctw")),
            Card::Country(Country::new("South Africa", 4,  "bcw")),
            Card::Country(Country::new("Zimbabwe",     2,  "cw")),
    
            // America
            Card::Country(Country::new("Bolivia",      8, "ctw")),
            Card::Country(Country::new("Brazil",       8, "bcw")),
            Card::Country(Country::new("Peru",         8, "ctw")),
            Card::Country(Country::new("Mexico",       6, "bc")),
            Card::Country(Country::new("Argentina",    4, "ctw")),
            Card::Country(Country::new("USA",          2, "bctw")),
            Card::Country(Country::new("Canada",       2, "ctw")),
    
            // Antarctica
            Card::Country(Country::new("Antarctica",   4, "tw")),
    
            // Asia
            Card::Country(Country::new("Mongolia",     10, "cw")),
            Card::Country(Country::new("China",        8,  "ctw")),
            Card::Country(Country::new("India",        8,  "bctw")),
            Card::Country(Country::new("Indonesia",    6,  "bctw")),
            Card::Country(Country::new("Nepal",        6,  "ctw")),
            Card::Country(Country::new("Uzbekistan",   6,  "ct")),
            Card::Country(Country::new("Thailand",     4,  "bc")),
            Card::Country(Country::new("Vietnam",      4,  "bc")),
            Card::Country(Country::new("Japan",        2,  "c")),
    
            // Europe
            Card::Country(Country::new("Russia",       6,  "ctw")),
            Card::Country(Country::new("Turkey",       6,  "bc")),
            Card::Country(Country::new("Italy",        4,  "ct")),
            Card::Country(Country::new("Germany",      2,  "c")),
            Card::Country(Country::new("Ireland",      2,  "c")),
            Card::Country(Country::new("UK",           2,  "c")),
            Card::Country(Country::new("France",       2,  "ct")),
            Card::Country(Country::new("Holland",      2,  "c")),
    
            // Oceania
            Card::Country(Country::new("Easter Island",6,  "c")),
            Card::Country(Country::new("Tahiti",       4,  "bcw")),
            Card::Country(Country::new("New Zealand",  4,  "bct")),
            Card::Country(Country::new("Australia",    4,  "bcw")),
            Card::Country(Country::new("Cook Islands", 2,  "bc")),
            Card::Country(Country::new("Fiji",         2,  "bcw")),
    
            // Bonus
            Card::Bonus(Bonus::Beach), Card::Bonus(Bonus::Beach), Card::Bonus(Bonus::Beach),
            Card::Bonus(Bonus::Culture), Card::Bonus(Bonus::Culture), Card::Bonus(Bonus::Culture),
            Card::Bonus(Bonus::Trekking), Card::Bonus(Bonus::Trekking),
            Card::Bonus(Bonus::Wildlife), Card::Bonus(Bonus::Wildlife),
    
            // Advice
            Card::Advice(Advice::new(true, AdviceType::Money)),
            Card::Advice(Advice::new(true, AdviceType::Money)),
            Card::Advice(Advice::new(true, AdviceType::Money)),
            Card::Advice(Advice::new(true, AdviceType::Transport)),
            Card::Advice(Advice::new(true, AdviceType::Transport)),
            Card::Advice(Advice::new(true, AdviceType::Transport)),
            Card::Advice(Advice::new(false, AdviceType::Transport)),
            Card::Advice(Advice::new(false, AdviceType::Transport)),
            Card::Advice(Advice::new(true, AdviceType::Timing)),
            Card::Advice(Advice::new(true, AdviceType::Timing)),
            Card::Advice(Advice::new(true, AdviceType::Timing)),
            Card::Advice(Advice::new(false, AdviceType::Timing)),
            Card::Advice(Advice::new(false, AdviceType::Timing)),
            Card::Advice(Advice::new(true, AdviceType::Bureaucracy)),
            Card::Advice(Advice::new(true, AdviceType::Bureaucracy)),
            Card::Advice(Advice::new(true, AdviceType::Bureaucracy)),
            Card::Advice(Advice::new(false, AdviceType::Bureaucracy)),
            Card::Advice(Advice::new(false, AdviceType::Bureaucracy)),

        ]
    }
}


// use tabular::{Table, Row};

impl Board {
    pub fn turn_heading(&self) {
        println!("--------------------------");
        println!("Its player {}'s turn", self.turn + 1);

        println!("Player {}'s hand:", self.turn + 1);
        self.players[self.turn].hand.iter().enumerate().for_each(|(i, card)| println!("| {} {}", i, card));

        println!();

        for (i, p) in self.players.iter().enumerate() {
            for (j, card) in p.pile.iter().enumerate() {
                if j == 0 { println!("Player {}", i) };
                println!("| {}", card);
            }
        }



        // let mut row_spec = String::new();
        // for _ in 0..self.players.len() {
        //     row_spec.push_str("| {:<}   ");
        // }

        // let mut table = Table::new(&row_spec);

        // let mut row = Row::new();
        // for i in 0..self.players.len() {
        //     row.add_cell(&format!("Player {}", i));
        // }
        // table.add_row(row);

        // for i in 0..self.players.iter().map(|p| p.pile.len()).max().unwrap_or(0) {
        //     let mut row = Row::new();
        //     for player in &self.players {
        //         let row_text = if let Some(country) = player.pile.get(i) {
        //             format!("{}", country)
        //         } else {
        //             "".to_string()
        //         };
        //         row.add_cell(row_text);
        //     }
        //     println!("{}", row.len());
        //     table.add_row(row);
        // }

        // println!("{}", table);
    }
}