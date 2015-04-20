BottomButton = React.createClass({
  mixins: [ReactMeteor.Mixin],

  getMeteorState: function () {

  },

  render: function () {
    return (
      <span>
        <div className="bottomButton">
          <div className="bottomButton fixed-action-btn">
            <a
              className="btn-floating btn-large waves-effect waves-light teal"
              href="#addProductModal"
              onClick={this.props.clickHandler}>
              <i className="mdi-content-add"></i>
            </a>
          </div>
        </div>

      </span>
    );
  }
});