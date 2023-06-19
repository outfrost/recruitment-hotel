# Assignment

Write a CLI program that will convert files from the INPUT format to the OUTPUT format, both
described below.

Your program should copy all the fields which are common between the two formats, and get all
the other data from `hotels.json` and `room_names.csv`.

Note: You must not modify any data files provided with the assignment.

Note 2: Please keep in mind that the input file (`input.csv`) is expected to be tens of gigabytes
in size, while `hotels.json` and `room_names.csv` are expected to be several hundred megabytes each.

## INPUT format

```
city_code|hotel_code|room_type|room_code|meal|checkin|adults|children|price|source
```

```
city_code  - three-letter city code
hotel_code - hotel code
room_type  - room type code
room_code  - particular room code
meal       - meal code
checkin    - checkin date
adults     - number of adults in the booking
children   - number of children in the booking
price      - total stay price
source     - price source name
```

## OUTPUT format

```
room_type meal;room_code;source;hotel_name;city_name;city_code;hotel_category;pax;adults;children;room_name;checkin;checkout;price
```

```
room_type meal - room type code and meal code, separated by a space
room_code      - particular room code
source         - price source name
hotel_name     - hotel name
city_name      - full name of the city
city_code      - three-letter city code
hotel_category - hotel category
pax            - total number of travellers in the booking
adults         - number of adults in the booking
children       - number of children in the booking
room_name      - room name
checkin        - checkin date
checkout       - checkout date (for this assignment, assume checkout == checkin + 1 day)
price          - total stay price PER PERSON
```

## hotels.json format

Note: Each line in this file contains a valid JSON object, but the entire file is not valid JSON.

```
{
  "id": "BER00003",                // hotel code
  "city_code": "BER",              // three-letter city code
  "name": "Berlin Marriott Hotel", // hotel name
  "category": 5.0,                 // hotel category
  "country_code": "DE",            // two-letter country code
  "city": "Berlin"                 // full name of the city
}
```

## room_names.csv format

```
hotel_code|source|room_name|room_code
```

```
hotel_code - hotel code
source     - price source name
room_name  - room name
room_code  - room code
```
