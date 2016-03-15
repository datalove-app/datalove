'use strict';

import React, {
  StyleSheet,
  Text,
  TouchableOpacity,
  View
} from 'react-native';
import Cycle from 'cycle-react/native';
import { makeGoto$, makeGoback$ } from '../../../lib/router.helpers.js';

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
  const session$ = props.get('session$');

  const scene$ = props.get('scene');
  const gohome$ = makeGoto$('/home', interactions.get('gohome'), scene$);
  const goback$ = makeGoback$(interactions.get('goback'), scene$);

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
