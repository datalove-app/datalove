'use strict';

import React, {
  Navigator,
  StyleSheet,
  Text,
  TouchableOpacity,
  View
} from 'react-native';
import { Observable } from 'rx';
import Cycle from 'cycle-react/native';

const Main = Cycle.component('Main', function computer(interactions) {
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

  return interactions.get('click')
    .map(ev => 1)
    .startWith(0)
    .scan((count, click) => count + click)
    .map((count) => {
      return (
        <View style={styles.container}>
          <TouchableOpacity onPress={interactions.listener('click')}>
            <Text style={styles.welcome}>
              hello world: {count}
            </Text>
          </TouchableOpacity>
        </View>
      );
    });
});

module.exports = Main;
