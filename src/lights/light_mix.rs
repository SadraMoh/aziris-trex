use druid::Data;

#[derive(Clone, Debug, Data, PartialEq)]
pub enum LightMix {
    Scroching, 
    Hot,       
    Warm,      
    Auto,      
    Cool,      
    Cold,      
    Freezing,  
}

impl Default for LightMix {
    fn default() -> Self {
        LightMix::Auto
    }
}