pub fn test_option_type() -> Option<String>{
 let   opt1: Option<String> = Some("kojjo".to_string());
  opt1
}

pub fn test_option_string() -> Option<String>{
 let opt1: Option<String> =  Some("JKKJKJK".to_string());
  opt1
}
pub fn test_option_charater() -> Option<CharacterType>{
    let  chartype =  Some(CharacterType::Mage);
    chartype
}

pub enum CharacterType {
    Archer,
    Warrior, 
    Mage
}
impl ToString for CharacterType{
  fn to_string(&self) -> String {
    match self {
       CharacterType::Archer => "Archer",
       CharacterType::Warrior => "Warrior",
       CharacterType::Mage => "Mage",

    }.to_string()
  }
}



































