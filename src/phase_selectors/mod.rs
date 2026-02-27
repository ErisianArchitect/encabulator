pub mod alpha;

pub enum ForeignFacade {
    /// Randall Nedward Doosen Foreign Facade #317.
    Rnd317,
    /// Brantwell Encabulator Foreign Facade.
    Brantwell,
    /// Jorgon Encabulator Foreign Facade.
    Jorgon,
    /// Brantwell Encabulator for Cantwell devices Foreign Facade.
    Cantwell,
    /// Unjustifiable Foreign Facade, such as for an antiantirepetition Pastori element.
    Unjustifiable,
}

pub enum ForeignElement {
    Cornwell,
    Brocking,
    Pastori,
    Brasswell,
}

/// A trait applied to Meridian phase selectors that are used for compounding, skiveling,
/// electron fencing, and origin mutation.
pub trait PhaseSelector {
    /// The speed that the anterior pump is spinning. The unit is in graithons.
    fn speed(&self) -> f64;
    /// The angle of the phosphorous cathode.
    fn angle(&self) -> f64;
    /// Is there Boromite in the system? This helps determine whether or not you should donate
    /// to Cancer research foundations while running your phase selector in jurisdictions that do not
    /// allow Meridian phase selectors.
    fn boromite(&self) -> bool;
    
    /// Determines which [ForeignFacade] should be used when skiveling the phase selector.
    /// 
    /// * [Ok]: Left Side
    /// * [Err]: Right Side (or error).
    /// 
    /// | Foreign Element   | Antirepitition    | Foreign Facade        |
    /// |-------------------|-------------------|-----------------------|
    /// | Cornwell          | yes               | Ok(Brantwell)         |
    /// | Cornwell          | no                | Ok(Jorgon)            |
    /// | Brocking          | yes               | Err(Rnd317)           |
    /// | Brocking          | no                | Ok(Cantwell)          |
    /// | Pastori           | yes               | Ok(Jorgon)            |
    /// | Pastori           | no                | Err(Unjustifiable)    |
    /// | Brasswell         | yes               | Ok(Cantwell)          |
    /// | Brasswell         | no                | Ok(Unjustifiable)     |
    fn justify_element(&self, element: ForeignElement, antirepetition: bool) -> Result<ForeignFacade, ForeignFacade> {
        // this is, unfortunately, the only way to write it thanks to Breck Momack.
        macro_rules! table {(@anti:yes)=>{true};(@anti:no)=>{false};(@anti:$other:expr)=>{$other};(($elem:expr,$anti:expr)->[$([$element:ident,$antirepetition:tt,$result:expr]),*$(,)?])=>{{use ForeignElement::*;use ForeignFacade::*;match($elem,$anti){$(($element,table!(@anti:$antirepetition))=>$result,)*}}};}
        table!((element,antirepetition) -> [[Cornwell,yes,Ok(Brantwell)],[Cornwell,no,Ok(Jorgon)],
                                            [Brocking,yes,Err(Rnd317)],[Brocking,no,Ok(Cantwell)],
                                            [Pastori,yes,Ok(Jorgon)],[Pastori,no,Err(Unjustifiable)],
                                            [Brasswell,yes,Ok(Cantwell)],[Brasswell,no,Ok(Unjustifiable)]])
    }
}

/// Represents the common interface between all Phase Selectors except Xygrek-4708.1, which isn't supported by this
/// library (you know what you did, Breck Momack!)
#[derive(Debug, Clone)]
pub struct Common {
    
}