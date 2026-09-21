// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {IGroth16Verifier} from "./IGroth16Verifier.sol";

interface IFamilyRegistry {
    function familyRoot() external view returns (uint256);
}

contract FamilyConstitution {
    IGroth16Verifier public immutable verifier;
    IFamilyRegistry public immutable registry;
    address public owner;
    address public agent;

    struct ActionState {
        uint256 tier;
        uint256 requiredApprovals;
        uint256 approvalCount;
        bool proposed;
        bool executed;
        mapping(uint256 => bool) usedNullifiers;
    }

    mapping(uint256 => ActionState) private actions;

    event ActionProposed(uint256 indexed actionId, uint256 tier, uint256 requiredApprovals);
    event ActionApproved(uint256 indexed actionId, uint256 approvalCount, uint256 requiredApprovals);
    event ActionAuthorized(uint256 indexed actionId);

    modifier onlyAgent() {
        require(msg.sender == agent, "not agent");
        _;
    }

    constructor(address verifierAddress, address registryAddress, address agentAddress) {
        verifier = IGroth16Verifier(verifierAddress);
        registry = IFamilyRegistry(registryAddress);
        owner = msg.sender;
        agent = agentAddress;
    }

    function requiredApprovalsForTier(uint256 tier) public pure returns (uint256) {
        if (tier == 0) return 0;
        if (tier == 1) return 1;
        if (tier == 2) return 2;
        if (tier == 3) return 3;
        revert("invalid tier");
    }
    function proposeAction(uint256 actionId, uint256 tier) external onlyAgent {
        ActionState storage a = actions[actionId];
        require(!a.proposed, "already proposed");

        uint256 required = requiredApprovalsForTier(tier);
        a.tier = tier;
        a.requiredApprovals = required;
        a.proposed = true;

        emit ActionProposed(actionId, tier, required);

        if (required == 0) {
            a.executed = true;
            emit ActionAuthorized(actionId);
        }
    }
    function approveAction(
        uint256 actionId,
        uint256[2] calldata pA,
        uint256[2][2] calldata pB,
        uint256[2] calldata pC,
        uint256[5] calldata pubSignals // [root, y, nullifier, epoch, challenge]
    ) external {
        ActionState storage a = actions[actionId];
        require(a.proposed, "action not proposed");
        require(!a.executed, "already authorized");

        uint256 root = pubSignals[0];
        uint256 nullifier = pubSignals[2];
        uint256 epoch = pubSignals[3];
        uint256 challenge = pubSignals[4];

        require(root == registry.familyRoot(), "Invalid root");
        require(epoch == (block.timestamp / 1 hours), "Invalid epoch");
        require(verifier.verifyProof(pA, pB, pC, pubSignals), "can not verified");
        require(challenge == actionId, "challenge is not for this action");
        require(!a.usedNullifiers[nullifier], "already approved by this member");

        a.usedNullifiers[nullifier] = true;
        a.approvalCount += 1;

        emit ActionApproved(actionId, a.approvalCount, a.requiredApprovals);

        if (a.approvalCount >= a.requiredApprovals) {
            a.executed = true;
            emit ActionAuthorized(actionId);
        }
    }
    function getActionState(uint256 actionId) 
        external
        view
        returns (uint256 tier, uint256 requiredApprovals, uint256 approvalCount, bool proposed, bool executed)
    {
        ActionState storage a = actions[actionId];
        return (a.tier, a.requiredApprovals, a.approvalCount, a.proposed, a.executed);
    }

}