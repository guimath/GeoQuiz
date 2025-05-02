import json
import numpy as np;
import os
from pathlib import Path
import geopandas as gpd
from matplotlib.patches import Polygon, Circle
from matplotlib.collections import PatchCollection

ABS_PATH = Path(__file__).parent

# BACKGROUND_COLOR = '#2a282d' 
# 146d00
BACKGROUND_COLOR = '#2a282d' 
'''Color, same as app background'''
SELECT_COLOR = '#26bd05'
'''Highlight color for maps (green)'''
LAND_COLOR = '#ffffff'
'''Color of land'''
LAT_LON_LINES_COLOR = '#ffffff70'
'''Color of the meridians and parallels'''

MAP_RATIO = 16/9
'''width to height ratio'''

FOLDER_GEOJSON = ABS_PATH/Path('geojson')
'''Folder with the combined geojson files'''
FILE_MAIN_INFOS = ABS_PATH/Path('countries.json')
'''Json with most infos to get ccas and names'''
FILE_MAIN_SHAPES = ABS_PATH/Path('countries.geojson')
'''geojson with higher quality shapes'''
FOLDER_OTHER_SHAPES = ABS_PATH/Path('flags')
'''folder with default geojson files'''
OUT_FOLDER_OUTLINES = (ABS_PATH/Path('../data/outlines')).resolve()
'''out folder where the generated outlines svgs are placed'''
OUT_FOLDER_MAPS = (ABS_PATH/Path('../data/maps')).resolve()
'''out folder where the generated maps svgs are placed'''

LON = 0
LAT = 1
def list_geojson_files():
    os.makedirs(FOLDER_GEOJSON, exist_ok=True)
    os.makedirs(OUT_FOLDER_OUTLINES, exist_ok=True)
    os.makedirs(OUT_FOLDER_MAPS, exist_ok=True)
    print(f'Maps to : {OUT_FOLDER_MAPS}')
    print(f'Outlines to : {OUT_FOLDER_OUTLINES}')

    with open(FILE_MAIN_INFOS, 'r') as f:
        infos = json.load(f)
    to_name = {i['cca3'].lower() : i['name']['common'] for i in infos}
    cca_list = list(to_name.keys())
    files = [f'{cca}.geo.json' for cca in cca_list]
    r_files = os.listdir(FOLDER_GEOJSON)
    if not set(files).issubset(set(r_files)) :
        print('Not all files geojson files present launching combine_files')
        combine_files()
    return [(cca_list[i], FOLDER_GEOJSON/files[i]) for i in range(len(files))]

def combine_files():
    IGN_MAIN = [
        "fra", # france is with all islands, we want only metropole
        "usa",
        "nor",
        "som",
        "esh",
        "mar",
        "cyp",
        "nld"
    ]
    os.makedirs(FOLDER_GEOJSON, exist_ok=True)
    with open(FILE_MAIN_INFOS, 'r') as f:
        infos = json.load(f)
    to_name = {i['cca3'].lower() : i['name']['common'] for i in infos}
    cca_list = list(to_name.keys())
    # print(cca_list)

    gdf = gpd.read_file(FILE_MAIN_SHAPES)
    for cca in cca_list : 
        
        if cca in IGN_MAIN :
            row = gpd.read_file(FOLDER_OTHER_SHAPES/f'{cca}.geo.json')
            row.to_file(FOLDER_GEOJSON/f'{cca}.geo.json')
            continue

        row = gdf.loc[gdf['ISO3166-1-Alpha-3'] == cca.upper()]
        if len(row) == 0 :
            row = gdf.loc[gdf['name'] == to_name[cca]]
            if len(row) == 0 :
                row = gpd.read_file(FOLDER_OTHER_SHAPES/f'{cca}.geo.json')

        row.to_file(FOLDER_GEOJSON/f'{cca}.geo.json')


def gen_patches_from_geojson(geojson_data, m, color):
    patches = []
    centers = []
    if geojson_data['type'] == 'FeatureCollection':
        for feature in geojson_data['features']:
            if feature['geometry']['type'] == 'Polygon':
                for polygon in feature['geometry']['coordinates']:
                    med = [0,0]
                    for lon, lat in polygon:
                        med[LON] += lon
                        med[LAT] += lat
                    med[LON]/=len(polygon)
                    med[LAT]/=len(polygon)
                    centers.append(med)
                    points = np.array([m(lon, lat) for lon, lat in polygon])
                    try : 
                        polygon = Polygon(points, closed=True)
                        
                        patches.append(polygon)
                    except:
                        pass
                    
            elif feature['geometry']['type'] == 'MultiPolygon':
                for multipolygon in feature['geometry']['coordinates']:
                    for polygon in multipolygon:
                        med = [0,0]
                        for lon, lat in polygon:
                            med[LON] += lon
                            med[LAT] += lat
                        med[LON]/=len(polygon)
                        med[LAT]/=len(polygon)
                        centers.append(med)
                        points = np.array([m(lon, lat) for lon, lat in polygon])
                        try : 
                            polygon = Polygon(points, closed=True)
                            patches.append(polygon)
                        except:
                            pass
    return (PatchCollection(
            patches, 
            facecolor=color, 
            edgecolor='black', 
            linewidth=0,
            zorder=2
        ),
        centers,
    )

if __name__ == '__main__':
    combine_files()