// SPDX-License-Identifier: MIT
pragma solidity ^0.7.0;

/// @title IBlockVerifier
abstract contract IBlockVerifier {
    function verifyProofs(
        uint[] calldata publicInputs,
        uint[] calldata proofs
    ) external virtual view returns (bool);
}
