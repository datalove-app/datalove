import "whuffie.peg.sol";

contract WhuffiePegMarket {
	mapping public (address tokenAddr => address pegAddr);

	function deposit() {
		/*
		 *  call original token's `send` to send tokens to WhuffieTokenPeg address
		 *  give WhuffieTokenPeg contract a limit of the original token's supply
		 *  WhuffieTokenPeg will send you the equivalent amount of credit
		 */
	}

	function withdraw() {
		/*
		 *  send as much credit as you want back to the WhuffieTokenPeg contract
		 *  WhuffieTokenPeg will then call original token's `send` to send you back the equivalent amount of tokens
		 */
	}
}
