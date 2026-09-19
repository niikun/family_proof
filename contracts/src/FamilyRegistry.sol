// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {IGroth16Verifier} from "./IGroth16Verifier.sol";

contract FamilyRegistry {
    IGroth16Verifier public immutable verifier;
    address public owner;
    uint256 public familyRoot;

    struct Signal {
        uint256 challenge;
        uint256 y;
        bool exists;
    }
    mapping(uint256 => Signal) public seenNullifiers; // nullifier => (challenge, y)

    event RootUpdated(uint256 newRoot);
    event ProofVerified(uint256 nullifier, uint256 epoch);
    event PotentialLeak(uint256 indexed nullifier, uint256 challenge1, uint256 y1, uint256 challenge2, uint256 y2, uint256 epoch);

    modifier onlyOwner() {
        require(msg.sender == owner, "not owner");
        _;
    }

    constructor(address verifierAddress, uint256 initialRoot) {
        verifier = IGroth16Verifier(verifierAddress);
        owner = msg.sender;
        familyRoot = initialRoot;
    }

    function updateRoot(uint256 newRoot) external onlyOwner {
        familyRoot = newRoot;
        emit RootUpdated(newRoot);
    }

    function verifyMembership(
        uint256[2] calldata pA,
        uint256[2][2] calldata pB,
        uint256[2] calldata pC,
        uint256[5] calldata pubSignals // [root, y, nullifier, epoch, challenge]
    ) external returns (bool) {
        uint256 root = pubSignals[0];
        uint256 y = pubSignals[1];
        uint256 nullifier = pubSignals[2];
        uint256 epoch = pubSignals[3];
        uint256 challenge = pubSignals[4];
        require(root == familyRoot,"Invalid root");
        require(epoch == (block.timestamp / 1 hours),"Invalid epoch");
        require(verifier.verifyProof(pA, pB, pC, pubSignals),"can not verified");
        Signal storage previous = seenNullifiers[nullifier];

        if (!previous.exists) {
            seenNullifiers[nullifier] = Signal({
                challenge: challenge, y: y, exists: true
            });
        } else if (previous.challenge == challenge) {
            revert("same challenge replay");
        } else {
            // 同じ nullifier・違う challenge = 使い回し検知
            emit PotentialLeak(nullifier, previous.challenge, previous.y, challenge, y, epoch);
        }
        emit ProofVerified(nullifier, epoch);
        return true;
    }
}