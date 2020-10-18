extern crate strum; // 0.10.0
#[macro_use]
extern crate strum_macros; // 0.10.0
use strum::AsStaticRef;
use rand::{
    Rng,
    distributions::{Distribution, Standard},
};
use rand::prelude::*;

#[derive(AsStaticStr)]
enum WpnAilment {
    None,
    Poison,
    Stun,               //A weapon can come with a status effect. Weapons can only have one status effect enabled.
    Freeze,             //The success rate of landing an effect is determined by the suffix in the item name.
    Death,
    Sleep
}

#[derive(AsStaticStr)]
enum WpnElement {
    None,
    Fire,
    Water,               //A weapon can come with a status effect. Weapons can only have one status effect enabled.
    Earth,             //The success rate of landing an effect is determined by the suffix in the item name.
    Light,
    Dark
}


#[derive(Debug, AsStaticStr)]
enum PoisonAliment {
    Bacteria = 10,
    Contaminated = 20,
    Tainted = 30,
    Toxic =  40,
    Infected = 50,
    Viral = 60,
    Venomous = 70,
    Virulent = 80,
    Noxious = 90, 
    Biohazardous = 100
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

#[derive(Debug, AsStaticStr)]
enum StunAliment {
    Tingling = 10,
    Numbing = 20,
    Stiffening = 30,
    Immobilizing =  40,
    Disabling = 50,
    Debilitating = 60,
    Paralyzing = 70,
    Disarming = 80,
    Arresting = 90, 
    Enfeebling = 100
}

impl Distribution<StunAliment> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> StunAliment {
        match rng.gen_range(0, 9) {
            0 => StunAliment::Tingling,
            1 => StunAliment::Numbing,
            2 => StunAliment::Stiffening,
            3 => StunAliment::Immobilizing,
            4 => StunAliment::Disabling,
            5 => StunAliment::Debilitating,
            6 => StunAliment::Paralyzing,
            7 => StunAliment::Disarming,
            8 => StunAliment::Arresting,
            9 => StunAliment::Enfeebling,
            _ => StunAliment::Tingling,
        }
    }
}



#[derive(Debug, AsStaticStr)]
enum FireElement {
    Heat = 10,
    Charring = 20,
    Embers = 30,
    Searing =  40,
    Bonfire = 50,
    Incandescence = 60,
    Brimstone = 70,
    Flames = 80,
    Combustion = 90, 
    Inferno = 100
}



// #[derive(Debug, AsStaticStr)]
// enum WpnElement {
//     None,
//     Fire,
//     Water,
//     Earth,              //If weapon rarity >= rare !legendary it can have an element. A staff is most likely to have an element than any other weapon type.
//     Wind,               //A weapon can have up to two elements, and the name of item will reflect that. The only invalid combinations
//     Light,              //are: Fire + Water, Earth + Wind, and Light + Dark.       
//     Dark 
// }

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

// impl Distribution<WpnAilment> for Standard {
//     fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> WpnAilment {
//         match rng.gen_range(0, 6) {
//             0 => WpnAilment::None,
//             1 => WpnAilment::Poison,
//             2 => WpnAilment::Stun,
//             3 => WpnAilment::Freeze,
//             5 => WpnAilment::Death,
//             6 => WpnAilment::Sleep,
//             _ => WpnAilment::None,
//         }
//     }
// }




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
    pub caracteristics_augmentation: Option<(String, u8)>,
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
    fn generate_ailment(&mut self, ailment : String) -> () {
        self.ailment = Some(ailment);
    }
    fn generate_ilvl() -> u8 {
        //TODO: ilvl should be related to item power
        //TODO: High ilvl should be rarest
        let mut rng = rand::thread_rng();
        rng.gen::<u8>()
    }
    fn generate_equip_type() -> String {
        let placeholders = [" sword ", " dagger ", " bow ", " mace ", " axe "];
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

    let mut itm = Weapon::generate();

    let wpn_ailment: WpnAilment = rand::random();

    if  itm.rarity == String::from("magic")  || itm.rarity == String::from("rare") || itm.rarity == String::from("epic")
    {
        let wap = match wpn_ailment {
            WpnAilment::Poison => {
                let wpn_alignment_power: PoisonAliment = rand::random();
                itm.generate_ailment(String::from(WpnAilment::Poison.as_static()));

                let mut name :String = wpn_alignment_power.as_static().to_owned();
                name.push_str(&itm.type_equip);
                itm.name =  Some(name); // Toxic dagger
                itm.caracteristics_augmentation = Some((String::from( WpnAilment::Poison.as_static()), wpn_alignment_power as u8));
            },
            WpnAilment::Stun => {
                let wpn_alignment_power: StunAliment = rand::random();
                itm.generate_ailment(String::from(WpnAilment::Stun.as_static()));

                let mut name :String = wpn_alignment_power.as_static().to_owned();
                name.push_str(&itm.type_equip);
                itm.name =  Some(name); // Toxic dagger
                itm.caracteristics_augmentation = Some((String::from( WpnAilment::Stun.as_static()), wpn_alignment_power as u8));
            },
            _ => (
                if itm.rarity == String::from("magic"){
                    let mut name :String = String::from("magic");
                    name.push_str(&itm.type_equip);
                    itm.name = Some(name);
                } else if itm.rarity == String::from("rare") {
                    let mut name :String = String::from("rare");
                    name.push_str(&itm.type_equip);
                    itm.name = Some(name);
                } else if itm.rarity == String::from("epic") {
                    let mut name :String = String::from("epic");
                    name.push_str(&itm.type_equip);
                    itm.name = Some(name);
                }
                
            )
    
        };

    } else if itm.rarity == String::from("common") {
        let mut name :String = String::from("common");
        name.push_str(&itm.type_equip);
        itm.name = Some(name);
    }

   

    

    println!("{:#?}", itm);
}
