
![](images/000_Title_Readme.png)

App still under dev - no official version 

# Screenshots 

|              |    |
:---------------------:|:-------------------:
![](images/100_SelectPlay.png)  |  ![](images/101_InfoSelect.png)
![](images/111_MainPlay.png)  |  ![](images/121_ChoicePlay.png)
![](images/200_LookUp.png)  |  ![](images/300_Score.png)


# Sources

|type| source | last update| Place to put sources|
|- | - | - | - |
| Geo json files | combined [mledoze github](https://github.com/mledoze/countries) and [Natural Earth](https://www.naturalearthdata.com/downloads/10m-cultural-vectors/) data via [datahub](https://datahub.io/core/geo-countries)  |  28/04/2025 |  [sources/flags](sources/flags) folder & [sources/countries.geojson](sources/countries.geojson)
| Population & Area | [United Nations](https://population.un.org/wpp/) via  [World Population Review](https://worldpopulationreview.com/)| 01/05/2025 | [sources/world_pop.csv](sources/worl_pop.csv) |
| Everything else | [mledoze github](https://github.com/mledoze/countries) | 10/03/2025 | [sources/countries.json](sources/countries.json) & [sources/flags](sources/flags) folder|


# Contact

Guilhem Mathieux [@guimath](https://github.com/guimath)
- dedicated mail: [guilhem.geoquiz@gmail.com](mailto:guilhem.geoquiz@gmail.com)



# Technical aspects

## Data Transformation

This app is made using rust. We used the slint framework for the front end.

Data from the different sources are processed in 2 steps:
1. Map generation
1. Aggregation



###  Maps generation 

Maps where generated using [matplotlib Basemap](https://matplotlib.org/basemap/stable/). Python scripts for the generation can be found in [maps.py](sources/maps.py) and [outlines.py](sources/outlines.py)

| Type | Projection | Generation date | 
| - | - | - | 
| Maps | [Azimuthal equidistant](https://en.wikipedia.org/wiki/Azimuthal_equidistant_projection) | 28/04/2025 | 
| Outlines | [Mercator](https://en.wikipedia.org/wiki/Mercator_projection) (except Antartica, Fiji, Kiribati & Russia in [Azimuthal equidistant](https://en.wikipedia.org/wiki/Azimuthal_equidistant_projection))  | 28/04/2025 | 

### Aggregation

Aggregation is done by the [prepare.rs]() file. It parses the different source files to produce a single Json that will be included in the compiled code.
It also includes the maps & outline as paths (not embedded images)
```sh
cargo run --example prepare
```

## Building

Once the map generation and data aggregation steps are done, you can build and run -> WIP

<!-- - For Desktop : ```cargo run```
- For Android : ```cargo run``` -->
