// Ensure all relevant items are marked with `pub` keyword
const CHRISTMAS_EMOJIS: [char; 4] = ['🎅', '🤶', '🎄', '🎁'];

// Your Solution here ...
pub trait Anonymize {
    fn anonymize_email(&self) -> String;
}

fn replace_with_emojis()

impl Anonymize for String {
    fn anonymize_email(&self) -> String {
        let result = &self.split_once('@');
        return match result {
            Some((local, domain)) => {
                let length = local.chars().count();
                let mut new_string = String::new();
                let mut index = 0;
                for _ in 0..length {
                    new_string.push(CHRISTMAS_EMOJIS[index]);
                    index += 1;
                    if index > 3 {
                        index = 0;
                    }
                }
                new_string + "@" + domain
            }
            None => {
                let mut new_string = String::new();
                let mut christmas_char_index = 0;
                for _ in 0..self.chars().count() {
                    new_string.push(CHRISTMAS_EMOJIS[christmas_char_index]);
                    christmas_char_index += 1;
                    if christmas_char_index > 3 {
                        christmas_char_index = 0;
                    }
                }
                new_string
            }
        };
    }
}

pub fn main() {
    let emails = vec![
        "rudolph.therapysessions@northpole.com".to_string(),
        "elfhr.complaint@northpole.urgent".to_string(),
        "santas.rage.management@christmaschaos.noel".to_string(),
        "overtimepay.never@elfexploitation.workshop".to_string(),
        "mrs.claus.divorce.lawyer@northpole.legal".to_string(),
        "reindeer.workers.comp@antler.insurance".to_string(),
        "naughty.list.revenge@santasecrets.com".to_string(),
        "workshop.ptsd.support@elves.anonymous".to_string(),
        "performance.anxiety@santa.breakdown".to_string(),
        "existential.crisis@northpole.void".to_string(),
    ];

    for email in emails {
        let anonymized_email = email.anonymize_email(); // This is the API that Santa wants!
        println!("Original: {} -> Anonymized: {}", email, anonymized_email);
    }
}
