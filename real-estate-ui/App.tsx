import * as React from 'react';
import { NavigationContainer } from '@react-navigation/native';
import { createNativeStackNavigator } from '@react-navigation/native-stack';
import Home from './components/Home';
import { Appbar, Text, Provider } from 'react-native-paper';
import { SafeAreaProvider } from 'react-native-safe-area-context';
import { View } from 'react-native';
import Navigation from './components/Navigation';
import SubdivisionsList from './components/SubdivisionsList';
import SubdivisionDetails from './components/SubdivisionDetails';
import SubdivisionsForm from './components/SubdivisionForm';
import LotForm from './components/LotForm';




const Stack = createNativeStackNavigator();

const App = () => {
  
  return (
    <Provider>
      <SafeAreaProvider>
        <NavigationContainer>
          <Stack.Navigator>
            <Stack.Screen 
              name="Real Estate"
              component={Home}
            />
            <Stack.Screen 
              name="Navigation"
              component={Navigation}
            />
            <Stack.Screen 
              name="Subdivisions"
              component={SubdivisionsList}
            />
            <Stack.Screen 
              name="SubdivisionsForm"
              component={SubdivisionsForm}
            />
            <Stack.Screen 
              name="SubdivisionsDetails"
              component={SubdivisionDetails}
            />
            <Stack.Screen 
              name="LotForm"
              component={LotForm}
            />
          </Stack.Navigator>
        </NavigationContainer>
      </SafeAreaProvider>
    </Provider>
  );
};

export default App;