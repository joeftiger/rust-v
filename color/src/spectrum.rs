use crate::cie::{xyz_of, CIE_Y_INTEGRAL};
use crate::color_data::*;
use crate::*;
use image::Rgb;

color!(
    Spectrum => LAMBDA_NUM, color_data::spectral
);

impl TryFrom<SerdeColors> for Spectrum {
    type Error = ();

    fn try_from(value: SerdeColors) -> Result<Self, Self::Error> {
        match value {
            SerdeColors::Spectrum(data) => Ok(Spectrum::new(data)),
            SerdeColors::Color(c) => Ok(Self::from(c)),
            SerdeColors::Constant(c) => Ok(Self::broadcast(c)),
            _ => Err(()),
        }
    }
}

impl From<Spectrum> for Srgb {
    fn from(spectrum: Spectrum) -> Self {
        Srgb::from(Xyz::from(spectrum))
    }
}

impl From<Spectrum> for Xyz {
    fn from(spectrum: Spectrum) -> Self {
        let xyz = spectrum
            .as_light_waves()
            .iter()
            .fold(Xyz::broadcast(0.0), |acc, next| {
                acc + xyz_of(next.lambda) * next.intensity
            });

        let scale = LAMBDA_RANGE / (CIE_Y_INTEGRAL * Spectrum::size() as Float);

        xyz * scale
    }
}

impl From<Spectrum> for Rgb<u8> {
    fn from(spectrum: Spectrum) -> Self {
        Self::from(Srgb::from(spectrum))
    }
}

impl From<Spectrum> for Rgb<u16> {
    fn from(spectrum: Spectrum) -> Self {
        Self::from(Srgb::from(spectrum))
    }
}

impl From<Spectrum> for Rgb<Float> {
    fn from(spectrum: Spectrum) -> Self {
        Self::from(Srgb::from(spectrum))
    }
}
