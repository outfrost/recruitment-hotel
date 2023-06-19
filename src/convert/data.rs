use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

mod input_date_format {
	use chrono::NaiveDate;
	use serde::{de::Error, Deserialize, Deserializer};

	pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<NaiveDate, D::Error> {
		let s: String = Deserialize::deserialize(deserializer)?;
		Ok(NaiveDate::parse_from_str(&s, "%Y%m%d").map_err(D::Error::custom)?)
	}
}

mod output_date_format {
	use chrono::NaiveDate;
	use serde::{Serialize, Serializer};

	pub fn serialize<S: Serializer>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error> {
		date.format("%Y-%m-%d").to_string().serialize(serializer)
	}
}

mod output_price_format {
	use serde::{Serialize, Serializer};

	pub fn serialize<S: Serializer>(price: &f64, serializer: S) -> Result<S::Ok, S::Error> {
		format!("{:.2}", price).to_string().serialize(serializer)
	}
}

#[derive(Deserialize)]
pub struct InputRecord {
	pub city_code: String,
	pub hotel_code: String,
	pub room_type: String,
	pub room_code: String,
	pub meal: String,
	#[serde(with = "input_date_format")]
	pub checkin: NaiveDate,
	pub adults: u32,
	pub children: u32,
	pub price: f64,
	pub source: String,
}

#[derive(Serialize)]
pub struct OutputRecord {
	#[serde(rename = "room_type meal")]
	pub room_type_with_meal: String,
	pub room_code: String,
	pub source: String,
	pub hotel_name: String,
	pub city_name: String,
	pub city_code: String,
	pub hotel_category: f32,
	pub pax: u32,
	pub adults: u32,
	pub children: u32,
	pub room_name: String,
	#[serde(with = "output_date_format")]
	pub checkin: NaiveDate,
	#[serde(with = "output_date_format")]
	pub checkout: NaiveDate,
	#[serde(with = "output_price_format")]
	pub price: f64,
}

#[derive(Deserialize)]
pub struct HotelRecord {
	pub id: String,
	pub city_code: String,
	pub name: String,
	pub category: f32,
	pub country_code: String,
	pub city: String,
}

#[derive(Deserialize)]
pub struct RoomRecord {
	pub hotel_code: String,
	pub source: String,
	pub room_name: String,
	pub room_code: String,
}

#[derive(PartialEq, Eq, Hash)]
pub struct HotelKey {
	pub hotel_code: String,
	pub city_code: String,
}

impl HotelKey {
	pub fn with_values(hotel_code: &str, city_code: &str) -> Self {
		Self {
			hotel_code: hotel_code.to_owned(),
			city_code: city_code.to_owned(),
		}
	}
}

pub struct HotelDetails {
	pub name: String,
	pub category: f32,
	pub city_name: String,
}

impl HotelDetails {
	pub fn with_values(name: &str, category: f32, city_name: &str) -> Self {
		Self {
			name: name.to_owned(),
			category,
			city_name: city_name.to_owned(),
		}
	}
}

#[derive(PartialEq, Eq, Hash)]
pub struct RoomKey {
	pub hotel_code: String,
	pub room_code: String,
	pub source: String,
}

impl RoomKey {
	pub fn with_values(hotel_code: &str, room_code: &str, source: &str) -> Self {
		Self {
			hotel_code: hotel_code.to_owned(),
			room_code: room_code.to_owned(),
			source: source.to_owned(),
		}
	}
}

pub struct RoomDetails {
	pub name: String,
}

impl RoomDetails {
	pub fn with_values(name: &str) -> Self {
		Self {
			name: name.to_owned(),
		}
	}
}
