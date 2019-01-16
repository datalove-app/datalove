import * as React from 'react';
import * as RX from 'reactxp';

const {
  Link,
  Styles,
  Text,
  View,
} = RX;

const styles = {
  container: Styles.createViewStyle({
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
    backgroundColor: '#f5fcff',
  }),
  helloWorld: Styles.createTextStyle({
    fontSize: 48,
    fontWeight: 'bold',
    marginBottom: 28,
  }),
  welcome: Styles.createTextStyle({
    fontSize: 32,
    marginBottom: 12,
  }),
  instructions: Styles.createTextStyle({
    fontSize: 16,
    color: '#aaa',
    marginBottom: 40,
  }),
  docLink: Styles.createLinkStyle({
    fontSize: 16,
    color: 'blue',
  }),
};

class App extends RX.Component<any, any> {
  private _animatedStyle: RX.Types.AnimatedTextStyleRuleSet;
  private _translationValue: RX.Animated.Value;

  public constructor(props: {}) {
    super(props);

    this._translationValue = new RX.Animated.Value(-100);
    this._animatedStyle = Styles.createAnimatedTextStyle({
      transform: [{
        translateY: this._translationValue,
      }],
    });
  }

  public componentDidMount() {
    const animation = RX.Animated.timing(this._translationValue, {
      toValue: 0,
      easing: RX.Animated.Easing.OutBack(),
      duration: 500,
    });

    animation.start();
  }

  public render() {
    return (
      <View style={styles.container}>
        <RX.Animated.Text style={[styles.helloWorld, this._animatedStyle]}>
          {'Hello World'}
        </RX.Animated.Text>

        <Text style={styles.welcome}>
          {'Welcome to ReactXP'}
        </Text>

        <Text style={styles.instructions}>
          {'Edit App.tsx to get started'}
        </Text>

        <Link // eslint-disable-line
          style={styles.docLink}
          url="https://microsoft.github.io/reactxp/docs"
        >
          {'View ReactXP documentation'}
        </Link>
      </View>
    );
  }
}

export default App;
