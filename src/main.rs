use rand::{
    Rng,
    distributions::{Distribution, Standard},
};
use rand::prelude::*;



enum WpnAilment {
    None,
    Poison,
    Stun,               //A weapon can come with a status effect. Weapons can only have one status effect enabled.
    Freeze,             //The success rate of landing an effect is determined by the suffix in the item name.
    Death,
    Sleep
}


#[derive(Debug)]
struct PoisonAliment {
    Bacteria :(String, u8),
    Contaminated :(String, u8),
    Tainted :(String, u8),
    Toxic :(String, u8),
    Infected :(String, u8),
    Viral :(String, u8),
    Venomous :(String, u8),
    Virulent :(String, u8),
    Noxious :(String, u8), 
    Biohazardous :(String, u8)
}


// enum PoisonAliment {
//     Bacteria = 10,
//     Contaminated = 20,
//     Tainted = 30,
//     Toxic =  40,
//     Infected = 50,
//     Viral = 60,
//     Venomous = 70,
//     Virulent = 80,
//     Noxious = 90, 
//     Biohazardous = 100
// }


enum WpnElement {
    None,
    Fire,
    Water,
    Earth,              //If weapon rarity >= rare !legendary it can have an element. A staff is most likely to have an element than any other weapon type.
    Wind,               //A weapon can have up to two elements, and the name of item will reflect that. The only invalid combinations
    Light,              //are: Fire + Water, Earth + Wind, and Light + Dark.       
    Dark 
}

impl Distribution<WpnAilment> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> WpnAilment {
        match rng.gen_range(0, 6) {
            0 => WpnAilment::None,
            1 => WpnAilment::Poison,
            2 => WpnAilment::Stun,
            3 => WpnAilment::Freeze,
            5 => WpnAilment::Death,
            6 => WpnAilment::Sleep,
            _ => WpnAilment::None,
        }
    }
}

impl Distribution<PoisonAliment> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PoisonAliment {
        match rng.gen_range(0, 9) {
            0 => PoisonAliment::Bacteria,
            1 => PoisonAliment::Contaminated,
            2 => PoisonAliment::Tainted,
            3 => PoisonAliment::Toxic,
            4 => PoisonAliment::Infected,
            5 => PoisonAliment::Viral,
            6 => PoisonAliment::Venomous,
            7 => PoisonAliment::Virulent,
            8 => PoisonAliment::Noxious,
            9 => PoisonAliment::Biohazardous,
            _ => PoisonAliment::Bacteria,
        }
    }
}


#[derive(Debug)]
pub struct Weapon {
    id: u32,
    pub name: Option<String>,
    ilevel: u8,
    type_equip: String, //Maybe pass a struct here
    tx_drop: f32,
    lvl_req: u8,
    rarity: String,
    min_damage: u8,
    max_damage: u8,
    pub element: Option<String>,
    pub ailment: Option<String>,
    pub caracteristics_augmentation: Option<String>,
    pub special: Option<String>
}

impl Weapon {
    fn generate() -> Weapon {
        Weapon {
            id: Weapon::generate_id(),
            name: None,
            ilevel: Weapon::generate_ilvl(),
            type_equip: Weapon::generate_equip_type(),
            tx_drop: 1.12,
            lvl_req: 60,
            rarity: Weapon::generate_rarity(),
            min_damage: Weapon::generate_min_damage(),
            max_damage: Weapon::generate_max_damage(),
            element: None, //
            ailment: None, //status effect. Weapons can only have one status effect enabled
            caracteristics_augmentation: None,
            special: Some(String::from("lol"))
        }
    }
    fn generate_id() -> u32 {
        let mut rng = rand::thread_rng();
        rng.gen::<u32>()
    }
    // fn generate_name() -> String {
    //     use rand::distributions::Alphanumeric;
    //     //TODO: use a dictionnary for naming
    //     let  rng = rand::thread_rng();
    //     return rng.sample_iter(&Alphanumeric)
    //     .take(10)
    //     .collect::<String>();
    // }
    fn generate_ilvl() -> u8 {
        //TODO: ilvl should be related to item power
        //TODO: High ilvl should be rarest
        let mut rng = rand::thread_rng();
        rng.gen::<u8>()
    }
    fn generate_equip_type() -> String {
        let placeholders = ["sword", "dagger"];
        placeholders.choose(&mut rand::thread_rng()).unwrap().to_string()
    }
    fn generate_rarity() -> String {
        let placeholders = ["common", "magic", "rare", "epic", "legendary"];
        placeholders.choose(&mut rand::thread_rng()).unwrap().to_string()
    }
    fn generate_min_damage() -> u8 {
        //TODO need to be boud to rarity and ilevel
        let mut rng = rand::thread_rng();
        rng.gen_range(0, 100)
    }
    fn generate_max_damage() -> u8 {
        //TODO need to be boud to rarity and ilevel
        let mut rng = rand::thread_rng();
        rng.gen_range(101, 255)
    }
   
}

fn main() {


    let itm = Weapon::generate();

    let wpn_ailment: WpnAilment = rand::random();

    let wap = match wpn_ailment {
        WpnAilment::Poison => {
            let wpn_alignment_power: PoisonAliment = rand::random();
            
            println!("Rectangle perimeter: {:#?} ", wpn_alignment_power,); // print Noxious
        },
        _ => ()

    };

    

    println!("Rectangle perimeter: {:#?}", itm);
}
