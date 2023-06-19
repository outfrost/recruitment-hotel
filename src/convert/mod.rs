use std::collections::HashMap;

use chrono::{Duration};

pub use data::*;

mod data;

pub struct Converter {
	hotels: HashMap<HotelKey, HotelDetails>,
	rooms: HashMap<RoomKey, RoomDetails>,
}

impl Converter {
	pub fn with_details<I, J>(hotel_records: I, room_records: J) -> Self
	where
		I: IntoIterator<Item = HotelRecord>,
		J: IntoIterator<Item = RoomRecord>,
	{
		let mut hotels = HashMap::new();
		let mut rooms = HashMap::new();

		for record in hotel_records {
			hotels.insert(
				HotelKey::with_values(&record.id, &record.city_code),
				HotelDetails::with_values(&record.name, record.category, &record.city));
		}

		for record in room_records {
			rooms.insert(
				RoomKey::with_values(&record.hotel_code, &record.room_code, &record.source),
				RoomDetails::with_values(&record.room_name));
		}

		Self { hotels, rooms }
	}

	pub fn transform(&self, input: InputRecord) -> Result<OutputRecord, String> {
		let pax = input.adults + input.children;

		let hotel = self.hotels
			.get(&HotelKey::with_values(&input.hotel_code, &input.city_code))
			.ok_or(format!(
				r#"Hotel details for hotel code "{}" and city code "{}" not found"#,
				&input.hotel_code,
				&input.city_code))?;

		let room = self.rooms
			.get(&RoomKey::with_values(&input.hotel_code, &input.room_code, &input.source))
			.ok_or(format!(
				r#"Room details for hotel code "{}" and room code "{}" and source "{}" not found"#,
				&input.hotel_code,
				&input.room_code,
				&input.source))?;

		Ok(OutputRecord {
			room_type_with_meal: format!("{} {}", input.room_type, input.meal),
			room_code: input.room_code,
			source: input.source,
			hotel_name: String::from(&hotel.name),
			city_name: String::from(&hotel.city_name),
			city_code: input.city_code,
			hotel_category: hotel.category,
			pax,
			adults: input.adults,
			children: input.children,
			room_name: String::from(&room.name),
			checkin: input.checkin,
			checkout: input.checkin + Duration::days(1),
			price: input.price / f64::from(pax),
		})
	}
}
