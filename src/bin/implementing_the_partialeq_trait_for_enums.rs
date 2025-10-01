enum Musician {
    SingerSongwriter(String),
    Band(u32)
}

use Musician::{Band, SingerSongwriter};

impl PartialEq for Musician {
    fn eq(&self, other: &Self) -> bool {
        match self {
            SingerSongwriter(name) => match other {
                SingerSongwriter(other_name) => name == other_name,
                Band(_) => false,
            },
            Band(members) => match other {
                SingerSongwriter(_) => false,
                Band(other_members) => members == other_members,
            }
        }
    }
    
}

fn main() {
    let rustin_bieber = SingerSongwriter("Rustin".to_string());
    let rustin_timberlake = SingerSongwriter("Rustin".to_string());
    let holly = SingerSongwriter("Holly".to_string());

    let rust_no_one = Band(5);
    let unrustworthy = Band(4);
    let rust_for_vengeance = Band(5);

    println!("{}", rustin_bieber == holly);
    println!("{}", rustin_bieber == rustin_timberlake);
    println!("{}", rustin_bieber == rust_no_one);

    println!("{}", rust_no_one == unrustworthy);
    println!("{}", rust_no_one == rust_for_vengeance);
}