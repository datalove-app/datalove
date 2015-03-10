AddProductModal = React.createClass({
  mixins: [ReactMeteor.Mixin],

  getMeteorState: function () {

  },

  addProduct: function(event) {
    event.preventDefault();
    console.log('adding product');

    Products.insert({
      name: ''
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
              ref="title"
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