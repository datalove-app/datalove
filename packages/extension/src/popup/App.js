"use strict";
var __extends = (this && this.__extends) || (function () {
    var extendStatics = function (d, b) {
        extendStatics = Object.setPrototypeOf ||
            ({ __proto__: [] } instanceof Array && function (d, b) { d.__proto__ = b; }) ||
            function (d, b) { for (var p in b) if (b.hasOwnProperty(p)) d[p] = b[p]; };
        return extendStatics(d, b);
    }
    return function (d, b) {
        extendStatics(d, b);
        function __() { this.constructor = d; }
        d.prototype = b === null ? Object.create(b) : (__.prototype = b.prototype, new __());
    };
})();
exports.__esModule = true;
var React = require("react");
var RX = require("reactxp");
var Link = RX.Link, Styles = RX.Styles, Text = RX.Text, View = RX.View;
var styles = {
    container: Styles.createViewStyle({
        flex: 1,
        justifyContent: 'center',
        alignItems: 'center',
        backgroundColor: '#f5fcff'
    }),
    helloWorld: Styles.createTextStyle({
        fontSize: 48,
        fontWeight: 'bold',
        marginBottom: 28
    }),
    welcome: Styles.createTextStyle({
        fontSize: 32,
        marginBottom: 12
    }),
    instructions: Styles.createTextStyle({
        fontSize: 16,
        color: '#aaa',
        marginBottom: 40
    }),
    docLink: Styles.createLinkStyle({
        fontSize: 16,
        color: 'blue'
    })
};
var App = /** @class */ (function (_super) {
    __extends(App, _super);
    function App(props) {
        var _this = _super.call(this, props) || this;
        _this._translationValue = new RX.Animated.Value(-100);
        _this._animatedStyle = Styles.createAnimatedTextStyle({
            transform: [{
                    translateY: _this._translationValue
                }]
        });
        return _this;
    }
    App.prototype.componentDidMount = function () {
        var animation = RX.Animated.timing(this._translationValue, {
            toValue: 0,
            easing: RX.Animated.Easing.OutBack(),
            duration: 500
        });
        animation.start();
    };
    App.prototype.render = function () {
        return (React.createElement(View, { style: styles.container },
            React.createElement(RX.Animated.Text, { style: [styles.helloWorld, this._animatedStyle] }, 'Hello World'),
            React.createElement(Text, { style: styles.welcome }, 'Welcome to ReactXP'),
            React.createElement(Text, { style: styles.instructions }, 'Edit App.tsx to get started'),
            React.createElement(Link // eslint-disable-line
            , { style: styles.docLink, url: "https://microsoft.github.io/reactxp/docs" }, 'View ReactXP documentation')));
    };
    return App;
}(RX.Component));
exports["default"] = App;
//# sourceMappingURL=App.js.map