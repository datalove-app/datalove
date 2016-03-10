'use strict';

import React, {
  StyleSheet,
  Text,
  TouchableOpacity,
  View
} from 'react-native';
import Cycle from 'cycle-react/native';
import { Observable } from 'rx';
import { Scene } from 'scene-router';

/* Need to require all components used to render routes */
import Auth from './Auth';
import Signin from './Auth/Signin';
import Signup from './Auth/Signup';
import Feed from './Feed';

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

const Home = Cycle.component('Home', function homeComputer(interactions, props) {
  const scene$ = props.get('scene');

  return interactions.get('click')
    .map(ev => 1)
    .startWith(0)
    .scan((count, click) => count + click)
    .combineLatest(scene$, (count, scene) => (
      <View style={styles.container}>
        <TouchableOpacity onPress={interactions.listener('click')}>
          <Text style={styles.welcome}>
            hello world: {count}
          </Text>
        </TouchableOpacity>

        <TouchableOpacity onPress={() => scene().goto('/auth/signin', {scene})}>
          <Text style={styles.welcome}>
            goto signin page
          </Text>
        </TouchableOpacity>

        <TouchableOpacity onPress={() => scene().goto('/auth/signup', {scene})}>
          <Text style={styles.welcome}>
            goto signup page
          </Text>
        </TouchableOpacity>
      </View>
    ));
});

const Main = Cycle.component('Main', function mainComputer(_1, props, self, _4, renderScheduler) {
  return props
    .observeOn(renderScheduler)
    .map(() => {
      const getScene = () => { return self.refs['scene']; }
      return (
        <Scene ref="scene"
          initialPath="/home"
          initialProps={{scene: getScene}}
          onSceneChange={(...args) => console.log('scene change', args)}
        >
          <Scene path="home" component={Home}></Scene>
          <Scene path="auth" component={Auth}>
            <Scene path="signin" component={Signin}></Scene>
            <Scene path="signup" component={Signup}></Scene>
          </Scene>
        </Scene>
      );
    });
}, {renderScheduler: true});

module.exports = Main;
