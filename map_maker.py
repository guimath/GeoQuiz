import matplotlib.pyplot as plt
from mpl_toolkits.basemap import Basemap # type: ignore
import geopandas as gpd
import os
from pathlib import Path
import json
from matplotlib.patches import Polygon, Circle
from matplotlib.collections import PatchCollection
import numpy as np
from haversine import haversine
from tqdm import tqdm

MAP_RATIO = 16/9
LAND_COLOR = '#ffffff' 
if False : #DARK
    LAKE_COLOR = '#2a282d'
    BACKGROUND = '#00000000'
else :
    LAKE_COLOR = '#90e0ef'
    BACKGROUND = '#90e0ef'
SELECT_COLOR = '#146d00'
MAP_QUALITY = 'i' # c < i < h
PROJECTION = 'aeqd'

TRANSPARENT = BACKGROUND[-2:] == '00' 
LON = 0
LAT = 1
EARTH_RADIUS = 6371000

def calc_dist(center, point):
    return haversine((center[LAT], center[LON]), (point[LAT],point[LON]), unit='m')

def gen_patches_from_geojson(geojson_data, m):
    patches = []
    centers = []
    if geojson_data['type'] == 'FeatureCollection':
        for feature in geojson_data['features']:
            if feature['geometry']['type'] == 'Polygon':
                for polygon in feature['geometry']['coordinates']:
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
            facecolor=SELECT_COLOR, 
            edgecolor='black', 
            linewidth=0,
            zorder=2
        ),
        centers,
    )
    

def main():
    folder = Path('data/flags')     
    folder2 = Path('data/positions2')
    ext_detect = '.geo.json'
    files = os.listdir(folder)
    files = list(filter(lambda x : x[-len(ext_detect):] == ext_detect, files))
    files.sort()
    # special_cases = ["ata","cok","cpv","fsm","gmb","gum","hmd","iot","jam","kir","lbn","mdv","mhl","mnp","mus","niu","pcn","pse","pyf","qat","rus","sgs","stp","tkl","ton","tto","tuv","wlf","wsm"]
    # files = [f'{s}.geo.json' for s in special_cases]
    for file in tqdm(files):
        fig = plt.figure(figsize=(10, 10/MAP_RATIO))#
        ax = fig.add_subplot(111)
        name = file[:-len(ext_detect)]
        # print(f'{name} --------------------')
        with open(folder/file, 'r') as f:
            geojson_data = json.load(f)
        gdf = gpd.GeoDataFrame.from_features(geojson_data)
        
        total_geometry = gdf.union_all()
        area = total_geometry.area
        centroid = total_geometry.centroid  
        min_lon, min_lat, max_lon, max_lat  = total_geometry.bounds
        center = [centroid.x, centroid.y]
        min = [min_lon, min_lat]
        max = [max_lon, max_lat]
        map_center = center.copy()
        
        # print(f'lon : {min[LON]}, {center[LON]}, {max[LON]}')
        # print(f'lat : {min[LAT]}, {center[LAT]}, {max[LAT]}')
        width = 5_000_000
        height = 1
        if name == 'umi':
            pass # default because on the edge so calc fail
        elif name == 'ata' :
            width = 10_000_000
            map_center[LON]= 0
            map_center[LAT]= -90
        elif name == 'cck' :
            width = 7_000_000
            map_center[LON]= 110
            map_center[LAT]= -10
        elif name == 'cok' : 
            width = 12_000_000
            map_center[LON]= -170
        elif name == 'fsm':
            width = 6_000_000
            map_center[LON]= 145
            map_center[LAT]= 0
        elif name == 'gum' :
            width = 7_000_000
            map_center[LON]= 135
        elif name == 'hmd' :
            width = 12_000_000
            map_center[LON]= 65
            map_center[LAT]= -40
        elif name == 'iot' :
            width = 7_000_000
            map_center[LAT]= 5
        elif name == 'kir':
            width = 12_000_000
            map_center[LON]= 170
            map_center[LAT]= -18
        elif name == 'mdv':
            width = 8_000_000
        elif name == 'mhl':
            width = 10_000_000
            map_center[LON]= 150
        elif name == 'mnp':
            width = 10_000_000
        elif name == 'niu':
            width = 10_000_000
            map_center[LON]= 170
        elif name == 'nru':
            width = 10_000_000
        elif name == 'pcn':
            width = 13_000_000
            map_center[LON]= -100
        elif name == 'pyf':
            width = 13_000_000
            map_center[LON]= 170
        elif name == 'rus' :
            width = 10_000_000
            map_center[LAT]= 70
        elif name == 'sgs' :
            width = 8_000_000
        elif name == 'tkl': 
            width = 10_000_000
            map_center[LON]= 170
        elif name == 'ton': 
            width = 10_000_000
        elif name == 'tuv':
            width = 10_000_000
        elif name == 'wlf':
            width = 10_000_000
        elif name == 'wsm':
            width = 10_000_000
        else : 
            a = haversine((center[LAT], center[LON]), (center[LAT],min[LON]), unit='m')*2*1.1
            b = haversine((center[LAT], center[LON]), (center[LAT],max[LON]), unit='m')*2*1.1
            width = np.max((a,b, 5_000_000))
            a = haversine((center[LAT], center[LON]), (min[LAT],center[LON]), unit='m')*2*1.1
            b = haversine((center[LAT], center[LON]), (max[LAT],center[LON]), unit='m')*2*1.1
            height = np.max((a,b))

        if width/height < MAP_RATIO :
            width = height*MAP_RATIO
        else : 
            height = width/MAP_RATIO
        # print(f"width: {width}, height: {height}")

        
        

        m = Basemap(
            projection=PROJECTION, 
            resolution=MAP_QUALITY,
            lat_0=map_center[LAT],
            lon_0=map_center[LON],
            width= width,
            height= height,
            )
        m.fillcontinents(color=LAND_COLOR, lake_color=LAKE_COLOR) # 2a282d
        m.drawmapboundary(fill_color=BACKGROUND)
        m.drawcoastlines(linewidth=0.25)
        m.drawcountries(linewidth=0.25)
        parallels = np.arange(-90., 91., 10.)
        m.drawparallels(parallels, labels=[0, 0, 0, 0])
        meridians = np.arange(-180., 181., 10.)
        m.drawmeridians(meridians, labels=[0, 0, 0, 0])
        pc, centers = gen_patches_from_geojson(geojson_data, lambda lon, lat : m(lon, lat))
        ax.add_collection(pc)
        # print(f"{name} - {area}")
        if area < 1 :
            center = [m(center[LON], center[LAT])]
            radius = [100000]
            if name == 'fsm' : 
                centers = [centers[0], centers[1], centers[2], centers[3], centers[7], centers[8], centers[14], centers[15], centers[18]]
                center = [m(c[LON], c[LAT]) for c in centers]
                radius = [80000 for _ in centers]
            elif name == 'cpv' :
                radius = [250000]
            elif name in ['gmb', 'jam', 'lbn', 'pse', 'qat', 'tto']:
                center=[]
            elif name == 'hmd' :
                radius= [200000]
            elif name in ['kir', 'pyf', 'stp', 'wlf']: 
                center = [m(c[LON], c[LAT]) for c in centers]
                radius = [80000 for _ in centers]
            elif name in ['mus', 'pcn', 'sgs' 'cok']:
                center = [m(c[LON], c[LAT]) for c in centers]
                radius = [100000 for _ in centers]
            elif name == 'tkl' : 
                radius = [200000]
            elif name == 'ton':
                centers = [centers[0], centers[2], centers[9], centers[-1]]
                center = [m(c[LON], c[LAT]) for c in centers]
                radius = [100000 for _ in center]
            elif name == 'tuv' : 
                radius = [270000]
            elif name == 'wsm' : 
                radius = [200000]
            for i in range(len(center)):
                circle = Circle(center[i], radius[i], ls="solid", lw=4, color="#146d00", fill = False)
                ax.add_patch(circle)
        ax.axis('off')
        plt.subplots_adjust(
            top=1.0,
            bottom=0, #0.06,
            left=0, #0.06,
            right=1.0,
            hspace=0.0,
            wspace=0.0
        )
        plt.savefig(folder2/(name+".svg"), format='svg', transparent=TRANSPARENT)
        # plt.show()
        plt.close('all')
if __name__ == "__main__" : main()