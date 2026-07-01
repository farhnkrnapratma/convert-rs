use actix_web::{HttpResponse, get, web};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Copy)]
enum TemperatureUnit {
    Celsius,
    Fahrenheit,
    Kelvin,
    Rankine,
    Reaumur,
}

#[derive(Serialize)]
struct TempResponse {
    from: char,
    to: char,
    val: f64,
    res: f64,
}

#[derive(Deserialize)]
pub struct TempParams {
    pub from: char,
    pub to: char,
    pub val: f64,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

impl TryFrom<char> for TemperatureUnit {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value.to_ascii_lowercase() {
            'c' => Ok(Self::Celsius),
            'f' => Ok(Self::Fahrenheit),
            'k' => Ok(Self::Kelvin),
            'r' => Ok(Self::Rankine),
            'e' => Ok(Self::Reaumur),
            _ => Err("Invalid unit. Accepted values: c, f, k, r, e".to_string()),
        }
    }
}

impl TemperatureUnit {
    fn to_celsius(self, value: f64) -> f64 {
        match self {
            Self::Celsius    => value,
            Self::Fahrenheit => ((value - 32.0) * 5.0) / 9.0,
            Self::Kelvin     => value - 273.15,
            Self::Rankine    => ((value - 491.67) * 5.0) / 9.0,
            Self::Reaumur    => (value * 5.0) / 4.0,
        }
    }

    fn from_celsius(self, value: f64) -> f64 {
        match self {
            Self::Celsius    => value,
            Self::Fahrenheit => ((value * 9.0) / 5.0) + 32.0,
            Self::Kelvin     => value + 273.15,
            Self::Rankine    => (value + 273.15) * 9.0 / 5.0,
            Self::Reaumur    => (value * 4.0) / 5.0,
        }
    }
}

#[get("/temperature")]
pub async fn convert_temperature(query: web::Query<TempParams>) -> HttpResponse {
    if !query.val.is_finite() {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: "val must be a finite number".to_string(),
        });
    }

    match calculate(query.from, query.to, query.val) {
        Ok(result) => HttpResponse::Ok().json(TempResponse {
            from: query.from,
            to: query.to,
            val: query.val,
            res: result,
        }),
        Err(err) => HttpResponse::BadRequest().json(ErrorResponse { error: err }),
    }
}

fn calculate(source: char, target: char, value: f64) -> Result<f64, String> {
    let source = TemperatureUnit::try_from(source)?;
    let target = TemperatureUnit::try_from(target)?;
    let celsius = source.to_celsius(value);
    Ok(target.from_celsius(celsius))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_celsius_to_fahrenheit() {
        assert_eq!(calculate('c', 'f', 50.0), Ok(122.0));
    }
}
