/* globals define */
define(function(require, exports, module) {
    'use strict';



    var Engine, Surface, Transform, StateModifier, mainContext, surface, modifier;
    Engine = require("famous/core/Engine");
    Surface = require("famous/core/Surface");
    Transform = require("famous/core/Transform");
    StateModifier = require("famous/modifiers/StateModifier");
    mainContext = Engine.createContext();
    surface = new Surface({
        size: [ 200, 500 ],
        content: "this is my text, bro",
        properties: {
            color: "black",
            textAlign: "center",
            backgroundColor: "#0FFFFF"
        }
    });
    modifier = new StateModifier({
        align: [ .5, .5 ],
        origin: [ .5, .5 ]
    });
    mainContext.add(modifier).add(surface);





});
