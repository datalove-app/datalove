YourMarket = React.createClass({
  mixins: [ReactMeteor.mixin],

  getMeteorState: function() {
    return {};
  },

  openModal: function() {
    $('#addProductModal').openModal();
  },

  render: function() {
    // finds only products you created based on productIds stored on your user obj...?
    var products = Products.find({});

    return (
      <div>
        <ul className="products collection">
          {products.map(function(product) {
            return <ProductListItem key={product._id} data={product} />
          })}
        </ul>

        <BottomButton onClick={this.openModal}/>
        <AddProductModal />
      </div>
    );
  }
});