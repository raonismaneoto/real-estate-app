import { useState } from "react";
import { Button, StyleSheet, View } from "react-native";
import MapView, { LatLng, Polygon } from "react-native-maps";

const App = () => {
  const [rotate, setRotate] = useState(true);
  const [scroll, setScroll] = useState(true);
  const [drawing, setDrawing] = useState(false);
  const [currDrawingCoordinates, setCurrentDrawingCoordinates] = useState<LatLng[]>([]);

  const handleOnPanDrag = (ev:any) => {
    if (drawing)  setCurrentDrawingCoordinates(currDrawingCoordinates.concat([ev.nativeEvent.coordinate as LatLng]));
  }

  return (
    <View>
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
      <View style={styles.panel}>
        {drawing ? (
          <>
            <Button onPress={() => {setRotate(true); setScroll(true); setDrawing(false); setCurrentDrawingCoordinates([])}} title="Click here to save the draw"/>
          </>
        ) : (
          <>
            <Button onPress={() => {setRotate(false); setScroll(false); setDrawing(true)}} title="Click here to draw the map"/>
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
    height: '20%',
    backgroundColor: 'white',
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
});

export default App;