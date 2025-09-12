fn main() {
    let f1 = FlavorRule {
        payment: 100,
        merit_value: 50,
        incense_value: 20,
    };
    let index = Flavor::StrongScent as u32;
    println!("index:{}", index);

    let config = &mut FlavorRulesConfig::new();
    config.setFlavor(Flavor::StrongScent, f1);

    println!("config: {:?}", config.rules[index as usize]);
}

pub enum Flavor {
    FaintScent = 0,
    StrongScent = 1,
    Spicy = 2,
}

#[derive(Debug)]
pub struct FlavorRule {
    pub payment: u32,
    pub merit_value: u32,
    pub incense_value: u32,
}

pub struct FlavorRulesConfig {
    pub rules: [FlavorRule; 3],
}

impl FlavorRulesConfig {
    pub fn new() -> FlavorRulesConfig {
        FlavorRulesConfig {
            rules: [
                FlavorRule {
                    payment: 0,
                    merit_value: 0,
                    incense_value: 0,
                },
                FlavorRule {
                    payment: 0,
                    merit_value: 0,
                    incense_value: 0,
                },
                FlavorRule {
                    payment: 0,
                    merit_value: 0,
                    incense_value: 0,
                },
            ],
        }
    }
    pub fn setFlavor(&mut self, flavor: Flavor, rule: FlavorRule) {
        self.rules[flavor as usize] = rule;
    }
}
