Meteor.methods({
  'addProduct': addProduct,

  'maxFlow': helpers.maxFlowBetweenAccounts
});

function addProduct(product) {
  Products.insert(product);
}
