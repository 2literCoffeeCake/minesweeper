#[derive(Clone, PartialEq)]
pub enum ColorSchema{
    BlueGray,
    Blue,
    Amber,
    Teal

}

impl From<String> for ColorSchema{
    fn from(str: String) -> Self {
        match str.to_lowercase().as_str(){
            "teal" => Self::Teal,
            "blue" => Self::Blue,
            "amber" => Self::Amber,
            "light-gray"| _ => Self::BlueGray
        }
    }
}

impl ToString for ColorSchema{
    fn to_string(&self) -> String {
        match self {
            ColorSchema::BlueGray => "light-gray",
            ColorSchema::Blue => "blue",
            ColorSchema::Amber => "amber",
            ColorSchema::Teal => "teal"
        }.to_string()
    }
}