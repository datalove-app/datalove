Meteor.methods({
  'addProduct': addProduct
});

function addProduct(product) {
  Products.insert(product);
}
