import { useEffect, useState } from "react";
import { StyleSheet, View } from "react-native";
import { Button, DataTable, Dialog, IconButton, Portal, Text } from 'react-native-paper';
import { TextInput } from 'react-native-paper';
import Navigation from "./Navigation";
import { noBodyRequest, request } from "../services/httpService";

const SubdivisionsList = () => {
  const [page, setPage] = useState<number>(0);
  const [numberOfItemsPerPageList] = useState([2, 3, 4]);
  const [itemsPerPage, onItemsPerPageChange] = useState(
    numberOfItemsPerPageList[0]
  );
  const [visible, setVisible] = useState(false);
  const [subdvisionName, setSubdivisionName] = useState('');
  const [subdvisionArea, setSubdivisionArea] = useState<[number,number][]>([]);
  const [showNavigation, setShowNavigation] = useState(false);
  const [subdivisions, setSubdivisions] = useState<Subdivision[]>([]);

  const from = page * itemsPerPage;
  const to = Math.min((page + 1) * itemsPerPage, subdivisions.length);

  useEffect(() => {
    setPage(0);
  }, [itemsPerPage]);

  useEffect(() => {
    const getSubdivisions = async () => {
      const response:any = await noBodyRequest('GET', 'subdivisions');

      if (response.error) {
        alert('error on subdivisions retrieval')
      } else {
        setSubdivisions(response.response.data);
      }
    }

    getSubdivisions();
  }, [])

  const onSubmit = async () => {
      const response:any = await request(
        'POST', 
        'subdivisions', 
        {name : subdvisionName, id: subdvisionName, area: subdvisionArea}
      );

      if (response.error) {
        alert('error on creation')
      } else {
        alert('criado com sucesso')
      }

      setVisible(false);
  };

  return (
    <View>
      {showNavigation ? 
        (
          <View>
            <Navigation extendedBehavior={true} setArea={setSubdivisionArea} changeControl={() => { setShowNavigation(false); setVisible(true); console.log(subdvisionArea); }}/>
          </View>
        ) 
        : 
        (
          <View>
            <DataTable>
              <DataTable.Header>
                  <DataTable.Title>Name</DataTable.Title>
                  <DataTable.Title numeric>Lots</DataTable.Title>
              </DataTable.Header>

              <DataTable.Row key="">
                  <DataTable.Cell>Mocked name</DataTable.Cell>
                  <DataTable.Cell numeric>0</DataTable.Cell>
                  </DataTable.Row>
              {subdivisions.slice(from, to).map((item) => (
                  <DataTable.Row key="">
                  <DataTable.Cell>{item.name}</DataTable.Cell>
                  <DataTable.Cell numeric>{item.lots.length}</DataTable.Cell>
                  </DataTable.Row>
              ))}

              <IconButton icon="plus" style={styles.newButton} onPress={() => setVisible(true)} mode="outlined" />

              <DataTable.Pagination
                  page={page}
                  numberOfPages={Math.ceil(subdivisions.length / itemsPerPage)}
                  onPageChange={(page) => setPage(page)}
                  label={`${from + 1}-${to} of ${subdivisions.length}`}
                  numberOfItemsPerPageList={numberOfItemsPerPageList}
                  numberOfItemsPerPage={itemsPerPage}
                  onItemsPerPageChange={onItemsPerPageChange}
                  showFastPaginationControls
                  selectPageDropdownLabel={'Rows per page'}
              />
            </DataTable>
            <Portal>
              <Dialog visible={visible} onDismiss={() => setVisible(false)}>
                <Dialog.Title>Subdivision Creation</Dialog.Title>
                <Dialog.Content>
                  <TextInput
                    label="Subdivision Name"
                    value={subdvisionName}
                    onChangeText={text => setSubdivisionName(text)} />
                  <Button mode="elevated" onPress={() => { setVisible(false); setShowNavigation(true) }} style={{alignSelf: 'flex-start', marginTop: 15}}>Select Area</Button>
                </Dialog.Content>
                <Dialog.Actions>
                  <Button onPress={onSubmit}>Submit</Button>
                  <Button onPress={() => setVisible(false)}>Cancel</Button>
                </Dialog.Actions>
              </Dialog>
            </Portal>
          </View>
      )}
    </View>
  );
}

const styles = StyleSheet.create({
    newButton: {
        alignSelf: "flex-end",
        width: "10%"
        
    }
})

export default SubdivisionsList;