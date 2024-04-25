import { useState } from "react";
import { Button, StyleSheet, View } from "react-native";
import MapView, { LatLng, Polygon } from "react-native-maps";
import { Searchbar } from 'react-native-paper';


interface NavigationProps {
  setArea?: (area: [number, number][]) => void,
  setShowNavigation?: (value: boolean) => void,
  extendedBehavior?: boolean
};

const Navigation = ({ setArea, setShowNavigation, extendedBehavior } : NavigationProps) => {
  const [rotate, setRotate] = useState(true);
  const [scroll, setScroll] = useState(true);
  const [drawing, setDrawing] = useState(false);
  const [currDrawingCoordinates, setCurrentDrawingCoordinates] = useState<LatLng[]>([]);
  const [searchQuery, setSearchQuery] = useState('');

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
      setShowNavigation && setShowNavigation(false);
      return;
    }

    setCurrentDrawingCoordinates([]);
  };


  return (
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
        >
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