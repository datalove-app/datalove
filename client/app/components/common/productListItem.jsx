ProductListItem = React.createClass({
  mixins: [ReactMeteor.Mixin],

  getMeteorState: function () {

  },

  render: function () {
    var imgUrl = "http://www.vanhoutte.com/design/en-ca/images/generic_product_large.gif";

    return (
        <div className="card small product-card">

          <div className="card-image waves-effect waves-block waves-light">
            <img className="product-image" src={imgUrl}/>
            <p>{this.props.data.name}</p>
          </div>

          <div className="card-content">
            <p>Price: ${this.props.data.price}</p>
            <p>{this.props.data.description}</p>
          </div>

        </div>
    );
  }
});