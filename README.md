# Geocoding API

![GitHub repo size](https://img.shields.io/github/repo-size/Stars-App/cities)
![Cargo version](https://img.shields.io/badge/version-0.1.0-blue)
![GitHub](https://img.shields.io/github/license/Stars-App/cities)

Cities is a simple API that we have developed to ease the search for locations while working with geographic names.

## Example

    curl -X GET "https://127.0.0.1/warszaw"

Response:

```json
[
    {
        "name": "Warszawice, Polska",
        "lat": 51.971389,
        "lon": 21.309746
    },
    {
        "name": "Warszawiaki, Polska",
        "lat": 51.10335,
        "lon": 22.3211
    },
    {
        "name": "Warszawa, Polska",
        "lat": 52.231958,
        "lon": 21.006725
    },
    {
        "name": "Dąbrówka Warszawska, Polska",
        "lat": 51.28694,
        "lon": 21.097779
    },
    {
        "name": "Warszawianka, Polska",
        "lat": 52.032007,
        "lon": 20.870612
    },
    {
        "name": "Warszawka, Polska",
        "lat": 53.015046,
        "lon": 19.541164
    },
    {
        "name": "Warszawka-Kolonia, Polska",
        "lat": 53.020879,
        "lon": 19.535608
    },
    {
        "name": "Kolonia Warszawska, Polska",
        "lat": 52.054387,
        "lon": 20.877507
    }
]
```

## Contributing

Contributions are always welcome!

- Fork the Project
- Create your Feature Branch (`git checkout -b feature/NewFeature`)
- Commit your Changes (`git commit -m 'Add a new feature'`)
- Push to the Branch (`git push origin feature/NewFeature`)
- Open a Pull Request

## License

Cities is licensed under the MIT license. See [license](https://github.com/Stars-App/cities/blob/master/LICENSE.md) for more details.