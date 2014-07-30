convertDate = function() {
	var day_zero = 946684800;
	var txn_time = this.date;
	var date = new Date((day_zero + txn_time) * 1000);
	return date;
};

submitXRPTxn = function(event, template) {
	event.preventDefault();

	var rcvrAddr = document.getElementById('recipient-addr').value;
	var amt = document.getElementById('send-amount').value;
	amt = Amount.from_human(amt + 'XRP');

	var tx = remote.transaction();

	tx.payment({
		from: Session.get('myAddr'),
		to: rcvrAddr,
		amount: amt
	});

	console.log('sending the txn...')

	tx.submit(function (err, res) {
		if (err) {
			console.log('error: ' + err.result_message);
		} else {
			console.log('successful txn submission!');
		}
	});
};