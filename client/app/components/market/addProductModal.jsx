AddProductModal = React.createClass({
  mixins: [ReactMeteor.Mixin],

  getMeteorState: function () {

  },

  addProduct: function(event) {
    event.preventDefault();
    console.log('adding product');

    var product = {
      name: this.refs.name.getDOMNode().value,
      description: this.refs.description.getDOMNode().value,
      price: parseFloat(this.refs.price.getDOMNode().value)
    };

    Meteor.call('addProduct', product, function(err, res) {
      if (err) { console.log('error adding product:', err); }
    });

    this.closeModal();
  },

  closeModal: function() {
    $('#addProductModal').closeModal();
  },

  render: function () {
    return (
      <div id="addProductModal" className="modal">
        <div className="modal-content">
          <form
            onSubmit={this.addProduct}>
            <input
              placeholder="Product Name"
              type="text"
              ref="name"
            />
            <input
              placeholder="Description"
              type="text"
              ref="description"
            />
            <input
              placeholder="Price"
              type="number"
              ref="price"
            />

            <input
              type="submit"
              className="hiddenSubmit"
            />
          </form>
        </div>
      </div>
    );
  }
});