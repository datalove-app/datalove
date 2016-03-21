'use strict';

import React, {
  LinkingIOS,
  StyleSheet,
  Text,
  TouchableOpacity,
  View
} from 'react-native';
import Cycle from 'cycle-react/native';
import Rx, { Observable } from 'rx';
import { Scene } from 'scene-router';
import { makeSceneRouter } from '../lib/router.helpers.js';

/* Need to require all components used to render routes */
import Home from './Home';
import Auth from './Auth';
import Signin from './Auth/Signin';
import Signup from './Auth/Signup';
import Feed from './Feed';

const Main = Cycle.component('Main', function mainComputer(_1, props, self, lifecycles, renderScheduler) {
  /*
    TODO: create a nav/scene stream that sets up a LinkingIOS listener and pipes paths
    linkingios stream functionality
      - receives a URL
      - checks auth, emits a route change object
    auth stream functionality:
      - checks some session storage
      - emits auth object based on auth status
    nav stream functionality:
      - should receive an object of:
        - desired path (so it can be processed for props)
        - props (to be merged with any global props to be passed to rendered component)
      - "subscribe" to the subject in the Main component
        - trigger scene change and pass props

    // in other components,
    ...
    scene.onNext({path: '/path'})

    // in main router component, something like this...
      // needs to merge props with global props to pass to new route
    const scene = new Rx.Subject();
    scene
      .combineLatest(auth$, (transition, auth) => {
        return transitionAction
      })
      .subscribe((action) => {
        
      })
      
   */

  let globalProps = {session$: new Rx.Subject()};
  globalProps.scene = makeSceneRouter(self, globalProps);
  /*
    TODO: return an object whose goto and goback methods emit an event
      - this way, we can intercept an intent to go to a path based on some criteria
      - so in principle:
        - map over the subject
        - merge it with the LinkingIOS and auth streams
        - plug it in to the vtree
   */

  return props
    .observeOn(renderScheduler)
    .map(() => {
      return (
        <Scene ref="scene"
          initialPath="/home"
          initialProps={{...globalProps}}
          onSceneChange={(...args) => {console.log(args)}}
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
