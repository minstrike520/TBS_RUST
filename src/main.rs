pub mod game {
    pub mod skills {
        pub struct SkillManager(Vec<String>);
        impl Default for SkillManager {
            fn default() -> Self {
                SkillManager(Vec::default())
            }
        }

        pub mod types {
            use crate::game;
            use game::character::{Character};
            use game::positioning::{Point};
            pub trait OnlyCaster {
                fn cast (_caster: &mut Character){}
            }
            pub trait HasOneTarget {
                fn cast (_caster: &mut Character, _target: &mut Character){}
            }
            pub trait HasMultipleTarget {
                fn cast (_caster: &mut Character, _targets: &mut Vec<Character>){}
            }
            pub trait HasLocation {
                fn cast (_caster: &mut Character, _targets: &Point){}
            }
        }
        use crate::game::character;
        pub struct Hit;
        impl types::OnlyCaster for Hit {
            fn cast (caster: &mut character::Character) {
                caster.hp.regen_percent(20);
            }
        }
    }
 
    pub mod positioning {
        pub struct Map();
        impl Default for Map {
            fn default() -> Self {
                Map()
            }
        }

        pub struct Point {
            x: i32,
            y: i32,
        }

        impl Point {
            fn new(x: i32, y: i32) -> Point {
                Point { x, y }
            }
        }

        trait IsMatrix {
            fn size(&self) -> Point;
        }

        impl IsMatrix for Map {
            fn size(&self) -> Point {
                Point::new(12, 12)
            }
        }

        pub struct LocationManager(Point);
        impl Default for LocationManager {
            fn default() -> Self {
                LocationManager(Point::new(0,0))
            }
        }
    }
    
    pub mod effects {
        #[derive(Debug)]
        pub enum Effect {
            AtkEnhanced,
        }
        #[derive(Debug)]
        pub struct EffectStatus {
            pub remain_turn: i32,
            pub effect_type: Effect,
        }
        pub struct EffectManager(Vec<EffectStatus>);
        impl Default for EffectManager {
            fn default() -> Self {
                EffectManager(Vec::default())
            }
        }

    }

    pub mod character {
        //use crate::game::skills;
        use crate::game;
        use game::flow::ActionValue;

        use super::{effects::EffectManager, positioning::LocationManager, skills::SkillManager};
        pub struct Sprite {
            location: game::positioning::LocationManager,
        }
        impl Default for Sprite {
            fn default() -> Self {
                Sprite { location: LocationManager::default() }
            }
        }
        pub struct CharacterInborn {
            pub atk: i32,
            pub def: i32,
            pub spd: i32,
        }
        impl Default for CharacterInborn {
            fn default() -> Self {
                CharacterInborn { atk: 100, def: 100, spd: 100 }
            }
        }

        pub struct Health(i32);
        impl Health {
            pub fn regen_percent(&mut self, percent: i32) {
                //TODO
            }
        }
        impl Default for Health {
            fn default() -> Self {
                Health(1000)
            }
        }
        pub struct Mana(i32);
        impl Default for Mana {
            fn default() -> Self {
                Mana(100)
            }
        }

        pub struct Character<'a> {
            pub id: String,
            pub effects: game::effects::EffectManager,
            pub sprite: Sprite,
            pub inborn: CharacterInborn,
            pub skills: game::skills::SkillManager,
            pub hp: Health,
            pub mp: Mana,
            pub action_value: ActionValue<'a>,
        }
        impl<'a> Default for Character<'a> {
            fn default() -> Self {
                Character { 
                    id: String::default(), 
                    effects: EffectManager::default(), 
                    sprite: Sprite::default(), 
                    inborn: CharacterInborn::default(), 
                    skills: SkillManager::default(), 
                    hp: Health::default(), 
                    mp: Mana::default(), 
                    action_value: ActionValue::default() }
            }
        }
    }

    pub mod flow {
        use crate::game;
        use std::mem;
        use game::character::Character;

        use super::positioning::Map;
        pub struct TurnCounter {
            pub count: i32,
        }
        impl TurnCounter {
            fn new ()-> Self {
                Self { count: (0) }
            }
            pub fn proceed(&mut self) {
                self.count += 1;
            }
        }
        pub struct ActionValue<'a> {
            pub value: i32,
            owner: &'a Character<'a>
        }
        impl<'a> ActionValue<'a> {
            pub fn new(owner: &'a Character)-> Self {
                Self { value: 1000, owner }
            }
            pub fn proceed (&mut self) {
                let speed: i32 = self.owner.inborn.spd;
                self.value -= speed;
            }
        }
        
        pub struct CharacterManager<'a> {
            content: Vec<Character<'a>>,
        }
        impl<'a> Default for CharacterManager<'a> {
            fn default() -> Self {
                CharacterManager { content: Vec::default() }
            }
        }
        impl<'a> CharacterManager<'a> {
            fn new() -> CharacterManager<'a>{
                CharacterManager{
                    content: Vec::new(),
                }
            }
            fn append(&mut self, char: Character<'a>) {
                self.content.push(char);                
            }
            fn action_orders_get_next(&mut self)-> i32 {
                0
                //TODO: impl get_next()
            }
        }


        struct TurnUnit<'a> {
            now_index: i32,
            characters: CharacterManager<'a>,
            map: Map,
        }
        impl<'a> TurnUnit<'a> {
            fn new(
                now_index: i32,
                characters: CharacterManager<'a>,
                map: Map
            ) -> TurnUnit<'a> {
                TurnUnit { now_index, characters, map }
            }
            fn execute(&mut self) {
                
            }
            fn give_back(&mut self, runtime: &mut Runtime<'a> ) {
                mem::swap(&mut self.characters,&mut runtime.characters);
            }
        }
        pub struct Runtime<'a> {
            is_started: bool,
            pub turn_counter: TurnCounter,
            pub characters: CharacterManager<'a>,
            pub map: Map,
        }
        impl<'a> Runtime<'a> {
            pub fn init () -> Runtime<'a> {
                Runtime { 
                    is_started: false, 
                    turn_counter: TurnCounter::new(), 
                    characters: CharacterManager::new(),
                    map: Map(),
                }
            }
            pub fn char_append (&mut self, char: Character<'a>) {
                assert!(!self.is_started);
                self.characters.append(char);
            }
            pub fn char_create () {

            }
            pub fn runtime_start (&mut self) {
                assert!(!self.characters.content.is_empty());
                let mut v: Vec<&ActionValue> = Vec::new();
                for p in self.characters.content.iter() {
                    v.push(&p.action_value);
                }

                self.is_started = true;
            }
            pub fn turn_execute(&mut self) {
                assert!(self.is_started);
                let characters = mem::take(&mut self.characters);

                let map = mem::take(&mut self.map);

                let this_turn: TurnUnit = TurnUnit::new(
                    self.characters.action_orders_get_next(),
                    characters,
                    map
                );

                /* 
                let this_turn: TurnUnit = TurnUnit::new(
                    self.characters.action_orders_get_next(),
                    &mut self.characters,
                    &mut self.map
                );*/
                //this_turn.execute();
            }
        }

    }
}

fn test_1 () {
    use crate::game::effects::{EffectStatus,Effect};
    let a = EffectStatus {
        effect_type: Effect::AtkEnhanced,
        remain_turn: 3,
    };
    println!("{:?}", a);
}
fn test_2 () {
    use crate::game::flow;
    let r = flow::Runtime::init();
    
    
}

fn main() {
    test_2();
}
