Template.txns.helpers({
	txnsList: function() {
		return Transactions.find({}, {sort: {date: -1}});
	}
});

Template.txn.helpers({
	age: function() {
		return convertDate
	},

	amount: function() {
		return this.amount/1e6
	}
});

Template.fam.rendered = function() {
	var Engine = require("famous/core/Engine");
	var Surface = require("famous/core/Surface");
	var View = require("famous/core/View");

	var mainContext = Engine.createContext();
	// var renderController = new View.RenderController();
	var surface = new Surface({
		content: 'Hello, meteor-famous!',
		size: [100, 100],
		properties: {
			color: 'white',
			textAlign: 'center',
			fontSize: '20px'
		}
	})

	// renderController.show(surface);

	mainContext.add(surface);

}

Template.sendSTR.events({
	'click input#submit-txn': submitSTRTxn
});

Template.config.helpers({
	myAddr: function() {
		return Session.get('myAddr')
	}
})

Template.config.events({
	'click input#submit-config' : updateConfig
})

Template.txnForms.events({

});