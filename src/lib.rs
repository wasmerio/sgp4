use std::sync::Mutex;
use wai_bindgen_rust::Handle;

use crate::sgp4::{
    EpochToSiderealTimeAlgorithm, Error as SgpError, ErrorTleLine, ErrorTleWhat, Geopotential,
    NegativeSemiLatusRectum, Orbit, OutOfRangeEccentricity, OutOfRangeEpochEccentricity,
    OutOfRangePerturbedEccentricity, Prediction, Tle,
};

use original::{self};

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
        Ok(original::Orbit::from_kozai_elements(
            &geopotential.into(),
            inclination,
            right_ascension,
            eccentricity,
            argument_of_perigee,
            mean_anomaly,
            kozai_mean_motion,
        )?
        .into())
    }

    fn wgs72() -> Geopotential {
        original::WGS72.into()
    }

    fn wgs84() -> Geopotential {
        original::WGS72.into()
    }

    fn afspc_epoch_to_sidereal_time(epoch: f64) -> f64 {
        original::afspc_epoch_to_sidereal_time(epoch)
    }

    fn iau_epoch_to_sidereal_time(epoch: f64) -> f64 {
        original::iau_epoch_to_sidereal_time(epoch)
    }

    fn parse2les(tles: String) -> Result<Vec<Handle<Elements>>, SgpError> {
        Ok(original::parse_2les(&tles)?
            .into_iter()
            .map(|e| Handle::new(e.into()))
            .collect())
    }

    fn parse3les(tles: String) -> Result<Vec<Handle<Elements>>, SgpError> {
        Ok(original::parse_2les(&tles)?
            .into_iter()
            .map(|e| Handle::new(e.into()))
            .collect())
    }
}

pub struct ResonanceState(Mutex<original::ResonanceState>);
impl ResonanceState {
    fn new(state: original::ResonanceState) -> Self {
        ResonanceState(Mutex::new(state))
    }
}

impl sgp4::ResonanceState for ResonanceState {
    fn t(&self) -> f64 {
        self.0.lock().expect("The mutex was poisioned").t()
    }
}

pub struct Constants(original::Constants);

impl Constants {
    fn new_wrap(state: original::Constants) -> Self {
        Constants(state)
    }
}

impl sgp4::Constants for Constants {
    fn new(
        geopotential: Geopotential,
        epoch_to_sidereal_time: EpochToSiderealTimeAlgorithm,
        epoch: f64,
        drag_item: f64,
        orbit0: Orbit,
    ) -> Result<Handle<Constants>, SgpError> {
        let epoch_to_sidereal_time_fn = match epoch_to_sidereal_time {
            EpochToSiderealTimeAlgorithm::Afspc => original::afspc_epoch_to_sidereal_time,
            EpochToSiderealTimeAlgorithm::Iau => original::iau_epoch_to_sidereal_time,
        };

        let state = original::Constants::new(
            geopotential.into(),
            epoch_to_sidereal_time_fn,
            epoch,
            drag_item,
            orbit0.into(),
        )?;

        Ok(Handle::new(Constants::new_wrap(state)))
    }

    fn initial_state(&self) -> Option<Handle<ResonanceState>> {
        self.0
            .initial_state()
            .map(|rs| Handle::new(ResonanceState::new(rs)))
    }

    fn propagate_from_state(
        &self,
        t: f64,
        state: Option<Handle<ResonanceState>>,
        afspc_compatibility_mode: bool,
    ) -> Result<Prediction, SgpError> {
        let mut rs = state
            .as_ref()
            .map(|state: &Handle<ResonanceState>| state.0.lock().expect("Lock was poisoned"));
        let rs = rs.as_deref_mut();

        Ok(self
            .0
            .propagate_from_state(t, rs, afspc_compatibility_mode)?
            .into())
    }

    fn propagate(&self, t: f64) -> Result<Prediction, SgpError> {
        Ok(self.0.propagate(t)?.into())
    }

    fn propagate_afspc_compatibility_mode(&self, t: f64) -> Result<Prediction, SgpError> {
        Ok(self.0.propagate_afspc_compatibility_mode(t)?.into())
    }

    fn from_elements(elements: Handle<Elements>) -> Result<Handle<Constants>, SgpError> {
        let state = original::Constants::from_elements(&elements.0)?;
        Ok(Handle::new(Constants::new_wrap(state)))
    }

    fn from_elements_afspc_compatibility_mode(
        elements: Handle<Elements>,
    ) -> Result<Handle<Constants>, SgpError> {
        let state = original::Constants::from_elements_afspc_compatibility_mode(&elements.0)?;
        Ok(Handle::new(Constants::new_wrap(state)))
    }
}

pub struct Elements(original::Elements);

impl Elements {
    fn new(state: original::Elements) -> Self {
        Elements(state)
    }
}

impl sgp4::Elements for Elements {
    fn from_tle(
        object_name: Option<String>,
        line1: String,
        line2: String,
    ) -> Result<Handle<Elements>, SgpError> {
        let state = original::Elements::from_tle(object_name, line1.as_bytes(), line2.as_bytes())?;
        Ok(Handle::new(Elements::new(state)))
    }

    fn epoch(&self) -> f64 {
        self.0.epoch()
    }

    fn epoch_afspc_compatibility_mode(&self) -> f64 {
        self.0.epoch_afspc_compatibility_mode()
    }
}

impl From<original::Prediction> for Prediction {
    fn from(prediction: original::Prediction) -> Self {
        let original::Prediction {
            position: [p1, p2, p3],
            velocity: [v1, v2, v3],
        } = prediction;
        Prediction {
            position: (p1, p2, p3),
            velocity: (v1, v2, v3),
        }
    }
}

impl From<Prediction> for original::Prediction {
    fn from(val: Prediction) -> Self {
        let Prediction {
            position: (p1, p2, p3),
            velocity: (v1, v2, v3),
        } = val;
        original::Prediction {
            position: [p1, p2, p3],
            velocity: [v1, v2, v3],
        }
    }
}

impl From<original::Orbit> for Orbit {
    fn from(o: original::Orbit) -> Self {
        let original::Orbit {
            inclination,
            right_ascension,
            eccentricity,
            argument_of_perigee,
            mean_anomaly,
            mean_motion,
        } = o;
        Orbit {
            inclination,
            right_ascension,
            eccentricity,
            argument_of_perigee,
            mean_anomaly,
            mean_motion,
        }
    }
}

impl From<Orbit> for original::Orbit {
    fn from(val: Orbit) -> Self {
        let Orbit {
            inclination,
            right_ascension,
            eccentricity,
            argument_of_perigee,
            mean_anomaly,
            mean_motion,
        } = val;
        original::Orbit {
            inclination,
            right_ascension,
            eccentricity,
            argument_of_perigee,
            mean_anomaly,
            mean_motion,
        }
    }
}

impl From<original::Geopotential> for Geopotential {
    fn from(original_geopotential: original::Geopotential) -> Self {
        let original::Geopotential { ae, ke, j2, j3, j4 } = original_geopotential;
        Geopotential { ae, ke, j2, j3, j4 }
    }
}
impl From<Geopotential> for original::Geopotential {
    fn from(val: Geopotential) -> Self {
        let Geopotential { ae, ke, j2, j3, j4 } = val;
        original::Geopotential { ae, ke, j2, j3, j4 }
    }
}

impl From<original::ErrorTleLine> for ErrorTleLine {
    fn from(e: original::ErrorTleLine) -> Self {
        match e {
            original::ErrorTleLine::Line1 => ErrorTleLine::Line1,
            original::ErrorTleLine::Line2 => ErrorTleLine::Line2,
            original::ErrorTleLine::Both => ErrorTleLine::Both,
        }
    }
}

impl From<original::ErrorTleWhat> for ErrorTleWhat {
    fn from(e: original::ErrorTleWhat) -> Self {
        match e {
            original::ErrorTleWhat::BadChecksum => ErrorTleWhat::BadChecksum,
            original::ErrorTleWhat::BadLength => ErrorTleWhat::BadLength,
            original::ErrorTleWhat::BadFirstCharacter => ErrorTleWhat::BadFirstCharacter,
            original::ErrorTleWhat::ExpectedFloat => ErrorTleWhat::ExpectedFloat,
            original::ErrorTleWhat::ExpectedFloatWithAssumedDecimalPoint => {
                ErrorTleWhat::ExpectedFloatWithAssumedDecimalPoint
            }
            original::ErrorTleWhat::ExpectedInteger => ErrorTleWhat::ExpectedInteger,
            original::ErrorTleWhat::ExpectedSpace => ErrorTleWhat::ExpectedSpace,
            original::ErrorTleWhat::ExpectedString => ErrorTleWhat::ExpectedString,
            original::ErrorTleWhat::FloatWithAssumedDecimalPointTooLong => {
                ErrorTleWhat::FloatWithAssumedDecimalPointTooLong
            }
            original::ErrorTleWhat::NoradIdMismatch => ErrorTleWhat::NoradIdMismatch,
            original::ErrorTleWhat::UnknownClassification => ErrorTleWhat::UnknownClassification,
            original::ErrorTleWhat::FromYoOptFailed => ErrorTleWhat::FromYoOptFailed,
            original::ErrorTleWhat::FromNumSecondsFromMidnightFailed => {
                ErrorTleWhat::FromNumSecondsFromMidnightFailed
            }
        }
    }
}

impl From<original::Error> for SgpError {
    fn from(e: original::Error) -> Self {
        match e {
            original::Error::OutOfRangeEpochEccentricity { eccentricity } => {
                SgpError::OutOfRangeEpochEccentricity(OutOfRangeEpochEccentricity { eccentricity })
            }
            original::Error::OutOfRangeEccentricity { eccentricity, t } => {
                SgpError::OutOfRangeEccentricity(OutOfRangeEccentricity { eccentricity, t })
            }
            original::Error::OutOfRangePerturbedEccentricity { eccentricity, t } => {
                SgpError::OutOfRangePerturbedEccentricity(OutOfRangePerturbedEccentricity {
                    eccentricity,
                    t,
                })
            }
            original::Error::NegativeBrouwerMeanMotion => SgpError::NegativeBrouwerMeanMotion,
            original::Error::NegativeKozaiMeanMotion => SgpError::NegativeKozaiMeanMotion,
            original::Error::NegativeSemiLatusRectum { t } => {
                SgpError::NegativeSemiLatusRectum(NegativeSemiLatusRectum { t })
            }
            original::Error::Tle {
                what,
                line,
                start,
                end,
            } => SgpError::Tle(Tle {
                what: what.into(),
                line: line.into(),
                start: start.try_into().unwrap(),
                end: end.try_into().unwrap(),
            }),
        }
    }
}

impl From<original::Elements> for Elements {
    fn from(elements: original::Elements) -> Self {
        Elements::new(elements)
    }
}

impl From<Elements> for original::Elements {
    fn from(elements: Elements) -> Self {
        elements.0
    }
}
