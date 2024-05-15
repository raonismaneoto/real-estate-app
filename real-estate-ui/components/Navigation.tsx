import { useEffect, useState } from "react";
import { Button, StyleSheet, View } from "react-native";
import MapView, { LatLng, Marker, Polygon, Region } from "react-native-maps";
import { Dialog, Divider, Portal, Searchbar, Text, TextInput } from 'react-native-paper';
import * as Location from "expo-location";
import { noBodyRequest, request } from "../services/httpService";

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
  const [showDetails, setShowDetails] = useState(false);
  const [selectedSubdivision, setSelectedSubdivision] = useState<Subdivision|undefined>(undefined);
  const [lotDrawing, setLotDrawing] = useState(false);
  const [showLotCreationDialog, setShowLotCreationDialog] = useState(false);
  const [creatingLotName, setCreatingLotName] = useState("");
  const [mapRef, setMapRef] = useState<MapView | null>(null);

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
    console.log(hasSearched);
    console.log(currentLocation.latitude);
    if (!hasSearched && currentLocation.latitude != 0) {
      searchSubdivisions();
    }
  }, [currentLocation])

  const searchSubdivisions = async (searchTerm: string | undefined = undefined) => {
    let searchQueryParam = "";
    if (searchTerm) {
      searchQueryParam = `name=${searchTerm}`;
    } else {
      searchQueryParam = `lat=${currentLocation.latitude}&long=${currentLocation.longitude}`;
    }
    console.log(`subdivisions/search?${searchQueryParam}`);
    const response : any = await noBodyRequest('GET', `subdivisions/search?${searchQueryParam}`);

    // console.log(response)
    // console.log(response.error);
    console.log(response.response.data);
    // console.log(response.message);

    if (response.response.data) {
      setSubdivisions(response.response.data);
      console.log(subdivisions);
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

  const saveLot = async () => {
    const reqBody = {
      area: currDrawingCoordinates.map(coords => [coords.latitude, coords.longitude]),
      name: creatingLotName,
      subdivision_id: selectedSubdivision?.id,
      id: creatingLotName
    }

    const result = await request('POST', `subdivisions/${selectedSubdivision?.id}/lots`, reqBody);

    if (result.error) {
      alert(result.response.data);
    } else {
      alert('Lot created successfully');
    }

    clearLotCreation();
  }

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

    if (lotDrawing) {
      // must open a pop-up to get lot name and confirm the creation request
      setShowLotCreationDialog(true);
      return;
    }

    setCurrentDrawingCoordinates([]);
  };

  const isInitialRegionSet = () => initialRegion.latitude !== 0 && initialRegion.longitude !== 0;

  const openSubdivisionDetails = (subdivision: Subdivision) => {
    console.log("oi")
    setSelectedSubdivision(subdivision);
    setShowDetails(true);
  }

  const drawLot = () => {
    setShowDetails(false);
    console.log(selectedSubdivision?.area[0]);
    const region : Region = {
      latitude: selectedSubdivision?.area[0][0] ? selectedSubdivision?.area[0][0] : currentLocation.latitude, 
      longitude: selectedSubdivision?.area[0][1] ? selectedSubdivision?.area[0][1] : currentLocation.longitude,
      latitudeDelta: 0.001, longitudeDelta: 0.001
    };
    mapRef?.animateToRegion(region, 2000);
    setLotDrawing(true);
  }

  const clearLotCreation = () => {
    setCurrentDrawingCoordinates([]);
    setCreatingLotName("");
    setSelectedSubdivision(undefined);
    setLotDrawing(false);
    setShowLotCreationDialog(false);
  }

  return (
    <>
      {isInitialRegionSet() && (
        <View>
        {
          !extendedBehavior ? (
            <Searchbar
              placeholder="Search"
              onChangeText={(value) => {setSearchQuery(value); setHasSearched(true)}}
              value={searchQuery}
              style={styles.searchBar}
              onIconPress={() => searchSubdivisions(searchQuery)}
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
            ref={ref => setMapRef(ref)}
          >
            <Marker 
              key="main"
              coordinate={{
                latitude: currentLocation.latitude,
                longitude: currentLocation.longitude,
              }}
              title="Your Location"
            />
            {subdivisions.length > 0 && subdivisions.map(each => (
              <Polygon
                key={each.id}
                coordinates={each.area.map(value => ({ latitude: value[0], longitude: value[1] } as LatLng))} 
                strokeColor="blue"
                strokeWidth={1}
                onPress={() => openSubdivisionDetails(each)}
                tappable={true}
              />
            )
            )}
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
        {lotDrawing? 
          (
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
          ) 
          : 
          (<></>)
        }
        
        {showDetails ? (
          <Portal>
            <Dialog visible={showDetails} onDismiss={() => setShowDetails(false)}>
              <Dialog.Title>Subdivision Details</Dialog.Title>
              <Dialog.Content>
                <Text variant="headlineSmall">Name: {selectedSubdivision?.name}</Text>
                <Divider />
                <Text variant="headlineSmall">Lots amount: {selectedSubdivision?.lots?.length || 0}</Text>
              </Dialog.Content>
              <Dialog.Actions>
                <Button onPress={() => setShowDetails(false)} title="Close"/>
                <Divider />
                <Button onPress={drawLot} title="Create lot"/>
              </Dialog.Actions>
            </Dialog>
          </Portal>
        ) : (<></>)}

        {showLotCreationDialog ? (
          <Portal>
            <Dialog visible={showLotCreationDialog} onDismiss={() => setShowLotCreationDialog(false)}>
              <Dialog.Title>Lot Creation</Dialog.Title>
              <Dialog.Content>
                <TextInput 
                  label="Lot Name/Id"
                  value={creatingLotName}
                  onChangeText={text => setCreatingLotName(text)}/>
              </Dialog.Content>
              <Dialog.Actions>
                <Button onPress={() => clearLotCreation()} title="Close"/>
                <Divider />
                <Button onPress={async () => await saveLot()} title="Create lot"/>
              </Dialog.Actions>
            </Dialog>
        </Portal>
        ) : (<></>)}
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