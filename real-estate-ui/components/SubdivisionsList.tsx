import { useEffect, useState } from "react";
import { StyleSheet, View } from "react-native";
import { Button, DataTable, Dialog, IconButton, Portal, Text } from 'react-native-paper';
import { TextInput } from 'react-native-paper';
import Navigation from "./Navigation";
import { request } from "../services/httpService";

const SubdivisionsList = () => {
  const [page, setPage] = useState<number>(0);
  const [numberOfItemsPerPageList] = useState([2, 3, 4]);
  const [itemsPerPage, onItemsPerPageChange] = useState(
    numberOfItemsPerPageList[0]
  );
  const [visible, setVisible] = useState(false);
  const [items] = useState([]);
  const [subdvisionName, setSubdivisionName] = useState('');
  const [subdvisionArea, setSubdivisionArea] = useState<[number,number][]>([]);
  const [showNavigation, setShowNavigation] = useState(false);

  const from = page * itemsPerPage;
  const to = Math.min((page + 1) * itemsPerPage, items.length);

  useEffect(() => {
    setPage(0);
  }, [itemsPerPage]);

  const onSubmit = async () => {
    try {
      const response = await request(
        'POST', 
        'subdivisions', 
        {name : subdvisionName, id: subdvisionName, area: subdvisionArea}
      );

      console.log(response);
      console.log(response.response);
      alert('criado com sucesso')
      setVisible(false);
    } catch (error:any) {
      console.log(error.response);
      alert(error)
    }
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
              {items.slice(from, to).map((item) => (
                  <DataTable.Row key="">
                  <DataTable.Cell>Mocked name</DataTable.Cell>
                  <DataTable.Cell numeric>0</DataTable.Cell>
                  </DataTable.Row>
              ))}

              <IconButton icon="plus" style={styles.newButton} onPress={() => setVisible(true)} mode="outlined" />

              <DataTable.Pagination
                  page={page}
                  numberOfPages={Math.ceil(items.length / itemsPerPage)}
                  onPageChange={(page) => setPage(page)}
                  label={`${from + 1}-${to} of ${items.length}`}
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