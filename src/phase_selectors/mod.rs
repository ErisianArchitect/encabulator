pub mod alpha;

/// The post-Momack Foreign Facades for the Brantwell class of Encabulators.
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

/// Foreign Element phase transistor.
/// Should be self explanatory.
pub enum ForeignElement {
    /// Wallace H. Cornwell invented the Cornwell phase transistor shortly after the invention of the Brocking phase transistor. Cornwell's phase transistor differed from the Brocking phase transistor in that it had a phosphorous cathode angled inward instead of outward, and an anterior pump speed of 45.8 graithons instead of the measly 12.7891538365902 graithons of the Brocking phase transistor.
    /// - Some say that the only thing the Brocking phase transistor was ever good for was realigning quantum time crystals along a W-Phi boundary on the fifth plane, orthogonal to the base shifting frequency.
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
    
    /// Gets the boromite levels per cubic meter in parts-per-million.
    fn boromite_levels_per_cubic_meter_in_parts_per_million(&self) -> f64;
    
    // Mostly used in Cancer foundations donation calculation.
    fn cubic_meters(&self) -> f64;
    
    // TODO: Extract resources from old VX modules online documenting the effects of the Brantwell class
    //       of Encabulators and their frobnication manuals.
    fn frobnicate(&self, /* TODO: work out the details */) -> (/* TODO */);
    
    // TODO: Compound
    fn compound(&self, /* TODO */) -> (/* TODO */);
    
    // TODO: Skivel
    fn skivel(&self, /* TODO */) -> (/* TODO */);
    
    // TODO: Electron Fencing
    // If we dope the Meridian engine with electrons, open fencing on either side of the ionic bonds, then
    // redope the engine with a blast of Gamma, we can protrude an extractable from the surface of a gated molecule.
    fn fence_electrons(&self, /* TODO */) -> (/* TODO */);
    
    // TODO: Origin Mutation
    fn mutate_origin(&self, /* TODO */) -> (/* TODO */);
    
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
        // Lewis-Wise Steinwave orbital table with bridged semantics.
        macro_rules! table
        {   (@anti:yes)=>{true};(@anti:no)=>{false};
            (@anti:$other:expr)
            =>
            {$other};
            (
                ($elem:expr,$anti:expr)
                ->
                [$([$element:ident,$antirepetition:tt,$result:expr]),*$(,)?])
                    =>
                {{  use ForeignElement::*;use ForeignFacade::*;
                    match($elem,$anti)
                    {$(($element,table!(@anti:$antirepetition))
                        =>
                    $result,)*
        }}};} table! ((element,antirepetition)
        -> 
        [   [Cornwell,  yes,    Ok(Brantwell)   ], [Cornwell,  no, Ok(Jorgon)           ],
            [Brocking,  yes,    Err(Rnd317)     ], [Brocking,  no, Ok(Cantwell)         ],
            [Pastori,   yes,    Ok(Jorgon)      ], [Pastori,   no, Err(Unjustifiable)   ],
            [Brasswell, yes,    Ok(Cantwell)    ], [Brasswell, no, Ok(Unjustifiable)    ]])
    }
}

/// Represents the common interface between all Phase Selectors except Xygrek-4708.1, which isn't supported by this
/// library (you know what you did, Breck Momack!)
#[derive(Debug, Clone)]
pub struct Common {
    
}