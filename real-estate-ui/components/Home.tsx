import * as React from 'react';
import { Button, StyleSheet, View } from 'react-native';

const Home = ({navigation}: {navigation:any}) => {
    console.log(navigation);

    return (
        <>
            <View>
                <View style={styles.button}>
                    <Button
                        title="Navigation"
                        onPress={() =>
                            navigation.push('Navigation')
                        }
                    />
                </View>
                <View style={styles.button}>
                    <Button
                        title="Subdivisions"
                        onPress={() =>
                            navigation.push('Subdivisions')
                        }
                    />
                </View>
            </View>
        </>
    );
}

const styles = StyleSheet.create({
    appBar: {
      alignItems: "center",
      justifyContent: "center",
    },
    button: {
        paddingBottom: 10
    }
});

export default Home;