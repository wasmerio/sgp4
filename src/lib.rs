use crate::sgp4::{
    Classification, ElementsS, EpochToSiderealTimeAlgorithm, Error as SgpError, ErrorTleLine,
    ErrorTleWhat, Geopotential, NegativeSemiLatusRectum, Orbit, OutOfRangeEccentricity,
    OutOfRangeEpochEccentricity, OutOfRangePerturbedEccentricity, Prediction, ResonanceState, Tle,
};

use original::{self};

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

    fn parse2les(tles: String) -> Result<Vec<ElementsS>, SgpError> {
        todo!()
        // let converted_vec: Vec<ElementsS> = original::parse_2les(&tles);
    }

    fn parse3les(tles: String) -> Result<Vec<ElementsS>, SgpError> {
        todo!()
    }

    fn resonance_state_t(rs: ResonanceState) -> f64 {
        rs.t
    }
}

struct Constants(original::Constants);

impl sgp4::Constants for Constants {
    fn new(
        geopotential: Geopotential,
        epoch_to_sidereal_time: EpochToSiderealTimeAlgorithm,
        epoch: f64,
        drag_item: f64,
        orbit0: Orbit,
    ) -> Result<Handle<Constants>, SgpError> {
        match epoch_to_sidereal_time {
            EpochToSiderealTimeAlgorithm::Afspc => match original::Constants::new(
                geopotential.into(),
                original::afspc_epoch_to_sidereal_time,
                epoch,
                drag_item,
                orbit0.into(),
            ) {
                Ok(res) => todo!(),
                Err(_) => todo!(),
            },
            EpochToSiderealTimeAlgorithm::Iau => todo!(),
        }
    }

    fn from_elements(
        elements: ElementsS,
    ) -> Result<wai_bindgen_rust::Handle<crate::Constants>, SgpError> {
        todo!()
    }

    fn from_elements_afspc_compatibility_mode(
        elements: ElementsS,
    ) -> Result<wai_bindgen_rust::Handle<crate::Constants>, SgpError> {
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

struct Elements(original::Elements);

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
        match original::Elements::from_tle(object_name, line1.as_bytes(), line2.as_bytes()) {
            Ok(elements) => Ok(Handle::new(Elements::new(elements))),
            Err(e) => Err(e.into()),
        }
    }

    fn epoch(&self) -> f64 {
        self.0.epoch()
    }

    fn epoch_afspc_compatibility_mode(&self) -> f64 {
        self.0.epoch_afspc_compatibility_mode()
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

impl Into<original::Orbit> for Orbit {
    fn into(self) -> original::Orbit {
        let Orbit {
            inclination,
            right_ascension,
            eccentricity,
            argument_of_perigee,
            mean_anomaly,
            mean_motion,
        } = self;
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
impl Into<original::Geopotential> for Geopotential {
    fn into(self) -> original::Geopotential {
        let Geopotential { ae, ke, j2, j3, j4 } = self;
        original::Geopotential { ae, ke, j2, j3, j4 }
    }
}

impl From<original::Classification> for Classification {
    fn from(classification: original::Classification) -> Self {
        match classification {
            original::Classification::Unclassified => Classification::Unclassified,
            original::Classification::Classified => Classification::Classified,
            original::Classification::Secret => Classification::Secret,
        }
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

impl From<original::Elements> for ElementsS {
    fn from(elements: original::Elements) -> Self {
        let original::Elements {
            object_name,
            international_designator,
            norad_id,
            classification,
            datetime,
            mean_motion_dot,
            mean_motion_ddot,
            drag_term,
            element_set_number,
            inclination,
            right_ascension,
            eccentricity,
            argument_of_perigee,
            mean_anomaly,
            mean_motion,
            revolution_number,
            ephemeris_type,
        } = elements;
        ElementsS {
            object_name,
            international_designator,
            norad_id,
            classification: classification.into(),
            datetime: datetime.to_string(),
            mean_motion_dot,
            mean_motion_ddot,
            drag_term,
            element_set_number,
            inclination,
            right_ascension,
            eccentricity,
            argument_of_perigee,
            mean_anomaly,
            mean_motion,
            revolution_number,
            ephemeris_type,
        }
    }
}
