User = Meteor.user();

Wall = React.createClass({
  mixins: [ReactMeteor.mixin],

  getInitialState: function() {
    return {
      userWithTxns: Session.get('user') || []
    }
  },

  render: function() {

    /*
    // TODO: WHY THE FUCK DO I HAVE TO DO THIS
      // WHY ISNT GETMETEORSTATE TAKING CARE OF THIS FOR ME???
      // AND WHY DOES IT ONLY WORK ON REFRESH /wall???

    var thisComponent = this;
    if (!this.state.userWithTxns) {
      console.log('no state');
      Meteor.users.find(Meteor.userId(), {
        fields: {
          transactions: 1
        }
      }).observe({
        changed: function(newData, oldData) {
          console.log('updating state');
          thisComponent.setState({userWithTxns: newData})
        }
      });
    }
     */

    var txns = this.state ? this.state.userWithTxns.receivedTransactions.reverse() : [];

    if (!txns.length) { return <div></div> }

    return (
      <div>
        <ul className="transactions collection">

          {txns.map(function(txn) {
            return (
              <TransactionListItem
                key={txn.txnHash}
                data={txn}
              />
            );
          }, this)}

        </ul>
      </div>
    );
  }
});