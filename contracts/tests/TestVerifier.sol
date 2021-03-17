pragma solidity ^0.7.0;

import "../deps/crypto/Verifier.sol";
import "../core/VerificationKeys.sol";

contract TestVerifier {
	function testVerify() public {
		(uint256[14] memory vk, uint256[4] memory vk_gammaABC) = VerificationKeys.getKey();

        return Verifier.Verify(vk, vk_gammaABC, in_proof, proof_inputs);
	}
}
