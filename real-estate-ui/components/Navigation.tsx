import { useEffect, useState } from "react";
import { Button, StyleSheet, View } from "react-native";
import MapView, { LatLng, Marker, Polygon, Region } from "react-native-maps";
import { Searchbar } from 'react-native-paper';
import * as Location from "expo-location";
import { noBodyRequest } from "../services/httpService";

interface NavigationProps {
  setArea?: (area: [number, number][]) => void,
  changeControl?: () => void,
  extendedBehavior?: boolean
};

const Navigation = ({ setArea, changeControl: setShowNavigation, extendedBehavior } : NavigationProps) => {
  const [rotate, setRotate] = useState(true);
  const [scroll, setScroll] = useState(true);
  const [drawing, setDrawing] = useState(false);
  const [currDrawingCoordinates, setCurrentDrawingCoordinates] = useState<LatLng[]>([]);
  const [searchQuery, setSearchQuery] = useState('');
  const [currentLocation, setCurrentLocation] = useState<Location.LocationObjectCoords>({latitude: 0, longitude: 0, altitude: null, accuracy: null, altitudeAccuracy: null,heading: null, speed: null});
  const [initialRegion, setInitialRegion] = useState<Region>({latitude: 0, longitude: 0, latitudeDelta: 0, longitudeDelta:0});
  const [subdivisions, setSubdivisions] = useState<Subdivision[]>([]);
  const [hasSearched, setHasSearched] = useState(false);

  useEffect(() => {
    const getLocation = async () => {
      let { status } = await Location.requestForegroundPermissionsAsync();

      if (status !== "granted") {
        console.log("Permission to access location was denied");
        return;
      }

      await Location.watchPositionAsync({
        accuracy:Location.Accuracy.Balanced,
        timeInterval: 10000,
        distanceInterval: 50,
      }, (newLocation) => {
        console.log(newLocation)
        setCurrentLocation(newLocation.coords);
        setInitialRegion({
          ...newLocation.coords,
          latitudeDelta: 0.1,
          longitudeDelta: 0.1,
        });
      });
    };

    getLocation();
  }, []);

  useEffect(() => {
    if (!hasSearched) {
      searchSubdivisions();
    }
  }, [currentLocation])

  useEffect(() => {
    if (!hasSearched) {
      setHasSearched(true);
    } 

    setTimeout(() => {
      searchSubdivisions(searchQuery);
    }, 3000);
  }, [hasSearched])

  const searchSubdivisions = async (searchTerm: string | undefined = undefined) => {
    let searchQueryParam = "";
    if (searchTerm) {
      searchQueryParam = `name=${searchTerm}`
    } else {
      searchQueryParam = `coords=${[currentLocation.latitude, currentLocation.longitude]}`
    }

    const response : any = noBodyRequest('GET', `subdivisions/search?${searchQueryParam}`)

    console.log(response);
    console.log(response.data);
    console.log(response.message);

    if (response.data) {
      setSubdivisions(response.data);
    }
  };

  const handleOnPanDrag = (ev:any) => {
    if (drawing) setCurrentDrawingCoordinates(currDrawingCoordinates.concat([ev.nativeEvent.coordinate as LatLng]));
  };

  const onStartDrawing = () => {
    setRotate(false); 
    setScroll(false); 
    setDrawing(true)
  };

  const onEndDrawing = () => {
    setRotate(true); 
    setScroll(true); 
    setDrawing(false); 
    
    if (extendedBehavior) {
      setArea && setArea(currDrawingCoordinates.map(coords => [coords.latitude, coords.longitude]));
      setCurrentDrawingCoordinates([]);
      setShowNavigation && setShowNavigation();
      return;
    }

    setCurrentDrawingCoordinates([]);
  };

  const isInitialRegionSet = () => initialRegion.latitude !== 0 && initialRegion.longitude !== 0;

  return (
    <>
      {isInitialRegionSet() && (
        <View>
        {
          !extendedBehavior ? (
            <Searchbar
              placeholder="Search"
              onChangeText={setSearchQuery}
              value={searchQuery}
              style={styles.searchBar}
            />
          ) : (<></>)
        }
        <View key="map">
          <MapView 
            style={styles.map} 
            onPanDrag={handleOnPanDrag}
            rotateEnabled={rotate}
            scrollEnabled={scroll}
            initialRegion={initialRegion}
          >
            <Marker 
              key="main"
              coordinate={{
                latitude: initialRegion.latitude,
                longitude: initialRegion.longitude,
              }}
              title="Your Location"
            />
            {currDrawingCoordinates.length > 0 ? (
              <>
                <Polygon 
                  coordinates={currDrawingCoordinates} 
                  strokeColor="blue"
                  strokeWidth={1}
                />
              </>
            ) : (<></>)}
          </MapView>
        </View>
        <View style={styles.panel}>
          {drawing ? (
            <>
              <Button onPress={onEndDrawing} title="Click here to save the draw"/>
            </>
          ) : (
            <>
              <Button onPress={onStartDrawing} title="Click here to draw the map"/>
            </>
          )}
        </View>
      </View>
      )}
    </>
  )
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    alignItems: "center",
    justifyContent: "center",
  },
  map: {
    width: "100%",
    height: "100%",
  },
  button: {
    flexDirection: 'column',
    alignItems: 'center',
  },
  panel: {
    flexDirection: 'column',
    bottom: '0%',
    width: '100%',
    height: '10%',
    position: 'absolute',
    borderTopLeftRadius: 24,
    borderTopRightRadius: 24,
    paddingTop: 12,
    paddingBottom: 12,
    paddingHorizontal: 24,
    shadowColor: '#000',
    shadowOffset: {
      width: 0,
      height: 2,
    },
    shadowOpacity: 0.25,
    shadowRadius: 3.84,
    elevation: 5,
  },
  searchBar: {
    position:'absolute',
    shadowColor: '#000',
    shadowOffset: {
      width: 0,
      height: 2,
    },
    shadowOpacity: 0.25,
    shadowRadius: 3.84,
    zIndex: 1,
    width: '90%',
    alignContent: 'center',
    alignItems: 'center',
    alignSelf: 'center',
    marginTop: '5%',
  }
});

export default Navigation;