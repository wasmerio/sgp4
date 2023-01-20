use crate::sgp4::{
    Classification, ElementsS, Error as SgpError, Geopotential, Orbit, Prediction, ResonanceState,
};

use wai_bindgen_rust::Handle;
wai_bindgen_rust::export!("sgp4.wai");

struct Sgp4;

impl sgp4::Sgp4 for Sgp4 {
    fn orbit_from_kozai_elements(
        geopotential: Geopotential,
        inclination: f64,
        right_ascension: f64,
        eccentricity: f64,
        argument_of_perigee: f64,
        mean_anomaly: f64,
        kozai_mean_motion: f64,
    ) -> Result<Orbit, SgpError> {
        todo!()
    }

    fn wgs72() -> Geopotential {
        todo!()
    }

    fn wgs84() -> Geopotential {
        todo!()
    }

    fn afspc_epoch_to_sidereal_time(epoch: f64) -> f64 {
        todo!()
    }

    fn iau_epoch_to_sidereal_time(epoch: f64) -> f64 {
        todo!()
    }

    fn parse2les(tles: String) -> Result<Vec<ElementsS>, SgpError> {
        todo!()
    }

    fn parse3les(tles: String) -> Result<Vec<ElementsS>, SgpError> {
        todo!()
    }

    fn resonance_state_t(rs: ResonanceState) -> f64 {
        todo!()
    }
}

struct Constants;

impl sgp4::Constants for Constants {
    fn new(
        geopotential: Geopotential,
        epoch_to_sidereal_time: sgp4::EpochToSiderealTimeAlgorithm,
        epoch: f64,
        drag_item: f64,
        orbit0: Orbit,
    ) -> Result<Handle<crate::Constants>, SgpError> {
        todo!()
    }

    fn from_elements(elements: ElementsS) -> Result<Handle<crate::Constants>, SgpError> {
        todo!()
    }

    fn from_elements_afspc_compatibility_mode(
        elements: ElementsS,
    ) -> Result<Handle<crate::Constants>, SgpError> {
        todo!()
    }

    fn initial_state(&self) -> Option<ResonanceState> {
        todo!()
    }

    fn propagate_from_state(
        &self,
        t: f64,
        state: Option<ResonanceState>,
        afspc_compatibility_mode: bool,
    ) -> Result<Prediction, SgpError> {
        todo!()
    }

    fn propagate(&self, t: f64) -> Result<Prediction, SgpError> {
        todo!()
    }

    fn propagate_afspc_compatibility_mode(&self, t: f64) -> Result<Prediction, SgpError> {
        todo!()
    }
}

struct Elements(ElementsS);

impl sgp4::Elements for Elements {
    fn from_tle(
        object_name: Option<String>,
        line1: String,
        line2: String,
    ) -> Result<Handle<crate::Elements>, SgpError> {
        todo!()
    }

    fn epoch(&self) -> f64 {
        todo!()
    }

    fn epoch_afspc_compatibility_mode(&self) -> f64 {
        todo!()
    }
}
