'use strict';

import React, {
  View
} from 'react-native';

import Signin from './Signin';
import Signup from './Signup';

export default (props) => {
  return (
    <View>
      <Signin scene={props.scene}/>
      <Signup scene={props.scene}/>
    </View>
  );
}
