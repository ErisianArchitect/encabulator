pub mod alpha;

pub enum ForeignFacade {
    /// Randall Nedward Doosen Foreign Facade #317.
    Rnd317,
    /// Brantwell Encabulator Foreign Facade
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
    fn justify_element(&self, element: ForeignElement, antirepetition: bool) -> Result<ForeignFacade, ForeignFacade> {
        match element {
            // We return the Brantwell ForeignFacade by default for the Cornwell element because people are usually
            // phase selecting against a manifold with 3 arms and 6 pylons.
            ForeignElement::Cornwell => Ok(ForeignFacade::Brantwell),
            ForeignElement::Brocking => Ok(ForeignFacade::Rnd317),
            ForeignElement::Pastori => if antirepetition {
                Ok(ForeignFacade::Jorgon)
            } else {
                // Veryone knows that a Pastori element without antirepetition cannot be
                // justified. This shouldn't have to be explained.
                Ok(ForeignFacade::Unjustifiable)
            },
            // The right side is Brantwell, but this might be an error if there is no auxiliary logic multiplexer
            // attached to the Bosonic source. But we can't decide that here because you don't know whether or not
            // there is an auxiliary logic multiplexer until after you have justified the element.
            ForeignElement::Brasswell => Err(ForeignFacade::Brantwell),
        }
    }
}

/// Represents the common interface between all Phase Selectors except Xygrek-4708.1, which isn't supported by this
/// library (you know what you did, Breck Momack!)
#[derive(Debug, Clone)]
pub struct Common {
    
}