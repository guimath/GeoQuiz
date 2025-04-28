import matplotlib.pyplot as plt
from mpl_toolkits.basemap import Basemap # type: ignore
import geopandas as gpd
import json
import shapely.geometry as sg
import shapely.ops as so
import numpy as np
from haversine import haversine
from tqdm import tqdm
import util as util

MAP_QUALITY = 'i' # c < i < h
PROJECTION = 'aeqd'

LON = util.LON
LAT = util.LAT

def calc_dist(center, point):
    return haversine((center[LAT], center[LON]), (point[LAT],point[LON]), unit='m')

def main():
    FIG_SIZE = 20
    l = util.list_geojson_files()
    # special_cases = ["ata","cok","cpv","fsm","gmb","gum","hmd","iot","jam","kir","lbn","mdv","mhl","mnp","mus","niu","pcn","pse","pyf","qat","rus","sgs","stp","tkl","ton","tto","tuv","wlf","wsm", "syc"]
    # special_cases = ['pri']
    # l = [(s, util.FOLDER_GEOJSON/f'{s}.geo.json') for s in special_cases]

    for cca, file in tqdm(l):
        fig = plt.figure(figsize=(FIG_SIZE, FIG_SIZE/util.MAP_RATIO))#
        ax = fig.add_subplot(111)
        with open(file, 'r') as f:
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
        # print(map_center)
        # print(f'lon : {min[LON]}, {center[LON]}, {max[LON]}')
        # print(f'lat : {min[LAT]}, {center[LAT]}, {max[LAT]}')
        width = 5_000_000
        height = 1
        if cca == 'ata' :
            width = 9_000_000
            map_center[LON]= 0
            map_center[LAT]= -90
        elif cca == 'cck' :
            width = 7_000_000
            map_center[LON]= 110
            map_center[LAT]= -10
        elif cca == 'cok' : 
            width = 12_000_000
            map_center[LON]= -170
        elif cca == 'fsm':
            width = 6_000_000
            map_center[LON]= 145
            map_center[LAT]= 0
        elif cca == 'gum' :
            width = 7_000_000
            map_center[LON]= 135
        elif cca == 'hmd' :
            width = 12_000_000
            map_center[LON]= 65
            map_center[LAT]= -40
        elif cca == 'iot' :
            width = 7_000_000
            map_center[LAT]= 5
        elif cca == 'kir':
            width = 12_000_000
            map_center[LON]= 170
            map_center[LAT]= -18
        elif cca == 'mhl':
            width = 10_000_000
            map_center[LON]= 150
        elif cca == 'niu':
            width = 10_000_000
            map_center[LON]= 170
        elif cca == 'pcn':
            width = 13_000_000
            map_center[LON]= -100
        elif cca == 'pyf':
            width = 13_000_000
            map_center[LON]= 170
        elif cca == 'rus' :
            width = 10_000_000
            map_center[LAT]= 70
        elif cca == 'tkl': 
            width = 10_000_000
            map_center[LON]= 170
        elif cca == 'usa':
            width = 12_000_000
            map_center[LAT]= 50
        elif cca == 'atf' :
            width = 13_000_000
            map_center[LON]= 40
            map_center[LAT]= -30
        elif cca == 'umi':
            width = 14_000_000
            map_center[LON] = -130
            map_center[LAT] = 20
        elif cca == 'mdv': width = 8_000_000
        elif cca == 'sgs': width = 8_000_000
        elif cca == 'aus': width = 9_000_000
        elif cca == 'mnp': width = 10_000_000
        elif cca == 'ton': width = 10_000_000
        elif cca == 'tuv': width = 10_000_000
        elif cca == 'nru': width = 10_000_000
        elif cca == 'wlf': width = 10_000_000
        elif cca == 'wsm': width = 10_000_000
        elif cca == 'bvt': width = 10_000_000
        else : 
            a = haversine((center[LAT], center[LON]), (center[LAT],min[LON]), unit='m')*2*1.1
            b = haversine((center[LAT], center[LON]), (center[LAT],max[LON]), unit='m')*2*1.1
            width = np.max((a,b, 5_000_000))
            a = haversine((center[LAT], center[LON]), (min[LAT],center[LON]), unit='m')*2*1.1
            b = haversine((center[LAT], center[LON]), (max[LAT],center[LON]), unit='m')*2*1.1
            height = np.max((a,b))

        if width/height < util.MAP_RATIO :
            width = height*util.MAP_RATIO
        else : 
            height = width/util.MAP_RATIO
        # print(f"width: {width}, height: {height}")

        m = Basemap(
            projection=PROJECTION, 
            resolution=MAP_QUALITY,
            lat_0=map_center[LAT],
            lon_0=map_center[LON],
            width= width,
            height= height,
        )
        m.fillcontinents(color=util.LAND_COLOR, lake_color=util.BACKGROUND_COLOR)
        m.drawmapboundary(fill_color=util.BACKGROUND_COLOR)
        # m.drawcoastlines(linewidth=0.25)
        m.drawcountries(linewidth=0.25)
        parallels = np.arange(-90., 91., 10.)
        m.drawparallels(parallels, labels=[0, 0, 0, 0], color=util.LAT_LON_LINES_COLOR)
        meridians = np.arange(-180., 181., 10.)
        m.drawmeridians(meridians, labels=[0, 0, 0, 0], color=util.LAT_LON_LINES_COLOR)
        pc, centers = util.gen_patches_from_geojson(geojson_data, lambda lon, lat : m(lon, lat), util.SELECT_COLOR)
        ax.add_collection(pc)
        # print(f"{cca} - {area}")
        if area < 1 :
            center = [m(c[LON], c[LAT]) for c in centers]
            radius = 100_000
            if cca in ['gmb', 'jam', 'lbn', 'pse', 'qat', 'tto', 'pri']:
                center=[] # no circle
            elif cca == 'tuv' : radius = 50_000
            elif cca == 'fsm' : radius = 80_000
            elif cca in ['kir', 'pyf', 'stp', 'wlf']: radius = 80_000
            elif cca == 'pcn' : radius = 150_000
            elif cca == 'hmd' : radius = 200_000
            elif cca == 'tkl' : radius = 200_000
            elif cca == 'wsm' : radius = 200_000
            elif cca == 'cpv' : radius = 250_000

            if len(center) > 0 :
                # using shapely to get union of circles
                circles = [sg.Point(c).buffer(radius) for c in center]
                union = so.unary_union(circles)
                if union.geom_type =="MultiPolygon":
                    shapes = list(union.geoms)
                else : 
                    shapes = [union]
                for shape in shapes:
                    xs, ys = shape.exterior.xy
                    ax.fill(xs, ys, fc="#00000000", ec=util.SELECT_COLOR, lw=4)

        ax.axis('off')
        plt.subplots_adjust(
            top=1.0,
            bottom=0,
            left=0,
            right=1.0,
            hspace=0.0,
            wspace=0.0
        )
        plt.savefig(util.OUT_FOLDER_MAPS/f'{cca}.svg', format='svg', transparent=True)
        # plt.show()
        plt.close('all')
if __name__ == "__main__" : main()