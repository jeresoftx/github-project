use bson::DateTime;
use chrono::Utc;

pub fn string_to_bson_datetime(
    date_string: &str,
) -> Result<bson::DateTime, Box<dyn std::error::Error>> {
    // Paso 1: Parsear el string a chrono::DateTime<Utc>
    // Asumimos formato ISO 8601 (como "2025-03-27T15:54:10Z")
    let chrono_date = match date_string.parse::<chrono::DateTime<Utc>>() {
        Ok(date) => date,
        Err(_) => return Err("Failed to parse date string".into()),
    };

    // Paso 2: Convertir chrono::DateTime a bson::DateTime
    let bson_date = DateTime::from_chrono(chrono_date);

    Ok(bson_date)
}
