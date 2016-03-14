'use strict';

import React, {
  StyleSheet,
  Text,
  TouchableOpacity,
  View
} from 'react-native';
import Cycle from 'cycle-react/native';

const styles = StyleSheet.create({
  container: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
    backgroundColor: '#F5FCFF',
  },
  welcome: {
    fontSize: 20,
    textAlign: 'center',
    margin: 5,
  }
});

export default Cycle.component('Signin', function(interactions, props) {

  const scene$ = props.get('scene');

  const gohome$ = interactions.get('gohome')
    .combineLatest(scene$, (_, scene) => {
      console.log('going home', scene);
      return scene().goto('/home', {scene});
    });
  const goback$ = interactions.get('goback')
    .combineLatest(scene$, (_, scene) => {
      console.log('going back fron signin', scene);
      return scene().goback();
    });

  const nav$ = gohome$.merge(goback$).startWith(null);

  return nav$.map(() => (
    <View style={styles.container}>
      <Text>signin page</Text>
      <TouchableOpacity onPress={interactions.listener('gohome')}>
        <Text style={styles.welcome}>go home</Text>
      </TouchableOpacity>
      <TouchableOpacity onPress={interactions.listener('goback')}>
        <Text style={styles.welcome}>go back</Text>
      </TouchableOpacity>
    </View>
  ));
})
