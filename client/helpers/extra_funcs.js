convertDate = function() {
	var day_zero = 946684800;
	var txn_time = this.date;
	var date = new Date((day_zero + txn_time) * 1000);
	return date;
};

ageString = function() {
	var day_zero = 946684800;
	var date = day_zero + this.date;
	var diff = (Date.now()/1000) - date;
	if (diff < 60) {
		return '< 1 minute ago';
	} else if (diff < 3600) {
		return Math.floor(diff/60) + ' minutes ago';
	} else if (diff < 86400) {
		if (diff < 7200) {
			return '1 hour ago'
		} else {
			return Math.floor(diff / 3600) + ' hours ago';
		}
	} else {
		return 'some time ago';
	}
},

submitSTRTxn = function(event, template) {
	// TODO: refactor this as a closure for both xrp and wfi txns

	event.preventDefault();
	var btn = $('#submit-txn');
	btn.button('loading');

	var input_rcvr = template.find('input[id=recipient-addr]');
	var input_amt = template.find('input[id=send-amount]');
	var rcvrAddr = input_rcvr.value.trim();
	var amt = input_amt.value;

	// console.log(rcvrAddr, amt);
	amt = Amount.from_human(amt + 'XRP');

	var tx = remote.transaction();

	tx.payment({
		from: Session.get('myAddr'),
		to: rcvrAddr,
		amount: amt
	});

	console.log('sending the txn...');

	tx.submit(function (err, res) {
		if (err) {
			console.log('error: ' + err.result_message);
		} else {
			console.log('successful txn submission!');
			input_rcvr.value = '';
			input_amt.value = '';
			btn.button('complete');
			Meteor.setTimeout(function() {
				btn.button('original');
			}, 5000);
		}
	});
};

updateConfig = function(event, template) {
	event.preventDefault();

	var input_addr = template.find('input[id=new-addr]');
	var input_key = template.find('input[id=new-key]');

	Session.set('myAddr', input_addr.value.trim());
	Session.set('mySecret', input_key.value);

	remote.set_secret(input_addr.value.trim(), input_key.value);

	input_addr.value = '';
	input_key.value = '';
}