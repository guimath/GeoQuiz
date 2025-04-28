import matplotlib.pyplot as plt
import json
import numpy as np;
from mpl_toolkits.basemap import Basemap # type: ignore
import geopandas as gpd
from tqdm import tqdm
import util



def main():
    FIG_SIZE = 20
    l = util.list_geojson_files()
    # special_cases = ['nzl']
    # l = [(s, util.FOLDER_GEOJSON/f'{s}.geo.json') for s in special_cases]
    for (cca, file) in tqdm(l):
        fig = plt.figure()#figsize=(FIG_SIZE, FIG_SIZE/util.MAP_RATIO))#
        ax = fig.add_subplot(111)
        with open(file, 'r') as f:
            geojson_data = json.load(f)
        gdf = gpd.GeoDataFrame.from_features(geojson_data)
        total_geometry = gdf.union_all()
        min_lon, min_lat, max_lon, max_lat  = total_geometry.bounds
        centroid = total_geometry.centroid  
        center = [centroid.x, centroid.y]
        if cca == 'usa': # ignoring guam sorry
            min_lon = -180 
            max_lon = -66.57
        elif cca == 'chl': # ignoring western most islands
            min_lon = -75.38
        elif cca == 'nzl': # ignoring small islands
            min_lon = 166
            max_lat = -34
            min_lat = -48


        if cca == 'ata': # antartica doesn't make sens in merc so using aeqd instead
            m = Basemap(
                projection='aeqd', 
                resolution='c',
                lat_0= -90,
                lon_0= 0,
                width= 5_600_000,
                height= 4_500_000,
            )
        # Otherwise use merc except when country is at the edge of 180W 180E 
        # (Russia, kiribati, fiji) -> using aeqd
        elif cca == 'rus':
            m = Basemap(
                projection='aeqd', 
                resolution='c',
                lon_0= 97,
                lat_0= 68,
                width= 9_000_000,
                height= 9_000_000/util.MAP_RATIO,
            )
        elif cca == 'kir': 
            m = Basemap(
                projection='aeqd', 
                resolution='c',
                lon_0= -170,
                lat_0= -2,
                width= 4_100_000,
                height= 4_100_000/util.MAP_RATIO,
            )
        elif cca == 'fji':
            m = Basemap(
                projection='aeqd', 
                resolution='c',
                lon_0= 180,
                lat_0= center[util.LAT],
                width= 2_000_000,
                height= 2_000_000/util.MAP_RATIO,
            )
        else :
            m = Basemap(
                projection='merc', 
                resolution='c',
                llcrnrlat=min_lat,
                urcrnrlat=max_lat,
                llcrnrlon=min_lon,
                urcrnrlon=max_lon,
            )
        
        m.drawmapboundary(fill_color=util.BACKGROUND_COLOR)
        parallels = np.arange(-90., 91., 10.)
        m.drawparallels(parallels, labels=[0, 0, 0, 0], color=util.LAT_LON_LINES_COLOR)
        meridians = np.arange(-180., 181., 10.)
        m.drawmeridians(meridians, labels=[0, 0, 0, 0], color=util.LAT_LON_LINES_COLOR)
        pc, centers = util.gen_patches_from_geojson(geojson_data, lambda lon, lat : m(lon, lat), util.LAND_COLOR)
        ax.add_collection(pc)
        # TODO Scale is a pain so none for now
        # m.drawmapscale(
        #     lon=max_lon,  # Longitude position of scale bar
        #     lat=max_lat,  # Latitude position of scale bar
        #     lon0=center[util.LON],   # Center longitude of map
        #     lat0=center[util.LAT],   # Center latitude of map
        #     length='auto',  # Length of scale bar in km
        #     barstyle='fancy',  # Style of scale bar ('simple' or 'fancy')
        #     units='km',  # Units of scale bar
        #     fontsize=12,  # Font size of scale bar text
        #     yoffset=100,  # Y-offset of scale bar
        #     labelstyle='simple'  # Style of scale bar labels
        # )
        # ax.add_artist(ScaleBar(1/110_574, box_alpha=0, font_properties={'size': 16}, color="#828284ff", rotation='horizontal-only'))
        ax.set_axis_off()
        plt.subplots_adjust(
            top=1.0,
            bottom=0,
            left=0,
            right=1.0,
            hspace=0.0,
            wspace=0.0
        )
        # plt.show()
        plt.savefig((util.OUT_FOLDER_OUTLINES)/f"{cca}.svg", format='svg', transparent=True, bbox_inches='tight')
        plt.close('all')

if __name__ == '__main__' :
    main()