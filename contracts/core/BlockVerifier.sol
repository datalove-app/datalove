import "./interfaces/IBlockVerifier.sol";

contract BlockVerifier is IBlockVerifier {
    function verifyProofs(
        uint256[] calldata publicInputs,
        uint256[] calldata proofs
    ) external view returns (bool) {
        return false;
    }
}
