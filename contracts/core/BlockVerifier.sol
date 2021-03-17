contract BlockVerifier {
    function verifyProofs(
        uint256[] calldata publicInputs,
        uint256[] calldata proofs
    ) external view returns (bool) {
        return false;
    }
}
