// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {Test} from "forge-std/Test.sol";
import {FamilyConstitution, IFamilyRegistry} from "../src/FamilyConstitution.sol";
import {IGroth16Verifier} from "../src/IGroth16Verifier.sol";

// 常に true を返すダミーの Verifier（FamilyRegistry.t.sol と同じもの）
contract MockVerifier is IGroth16Verifier {
    function verifyProof(
        uint256[2] calldata,
        uint256[2][2] calldata,
        uint256[2] calldata,
        uint256[5] calldata
    ) external pure returns (bool) {
        return true;
    }
}

// FamilyConstitutionが参照するFamilyRegistryの代わり、固定rootを返すだけ
contract MockRegistry is IFamilyRegistry {
    uint256 public root;
    constructor(uint256 _root) { root = _root; }
    function familyRoot() external view returns (uint256) { return root; }
}

contract FamilyConstitutionTest is Test {
    FamilyConstitution constitution;
    uint256 constant ROOT = 111;
    address constant AGENT = address(0xA6E47);
    uint256 constant ACTION_ID = 42;

    uint256[2] pA;
    uint256[2][2] pB;
    uint256[2] pC;

    function setUp() public {
        MockVerifier verifier = new MockVerifier();
        MockRegistry registry = new MockRegistry(ROOT);
        constitution = new FamilyConstitution(address(verifier), address(registry), AGENT);
    }

    function _pubSignals(uint256 y, uint256 nullifier, uint256 epoch, uint256 challenge)
        internal pure returns (uint256[5] memory)
    {
        return [ROOT, y, nullifier, epoch, challenge];
    }

    function test_tier0_executesImmediately() public {
        vm.prank(AGENT);
        constitution.proposeAction(ACTION_ID, 0);

        (, , , , bool executed) = constitution.getActionState(ACTION_ID);
        assertTrue(executed);
    }

    function test_tier2_notExecutedAfterOneApproval() public {
        vm.prank(AGENT);
        constitution.proposeAction(ACTION_ID, 2);

        uint256 epoch = block.timestamp / 1 hours;
        constitution.approveAction(ACTION_ID, pA, pB, pC, _pubSignals(1, 999, epoch, ACTION_ID));

        (, , uint256 approvalCount, , bool executed) = constitution.getActionState(ACTION_ID);
        assertEq(approvalCount, 1);
        assertFalse(executed);
    }

    function test_tier2_executesAfterSecondApproval() public {
        vm.prank(AGENT);
        constitution.proposeAction(ACTION_ID, 2);

        uint256 epoch = block.timestamp / 1 hours;
        constitution.approveAction(ACTION_ID, pA, pB, pC, _pubSignals(1, 999, epoch, ACTION_ID));

        vm.expectEmit(true, false, false, true);
        emit FamilyConstitution.ActionAuthorized(ACTION_ID);
        constitution.approveAction(ACTION_ID, pA, pB, pC, _pubSignals(2, 888, epoch, ACTION_ID));

        (, , , , bool executed) = constitution.getActionState(ACTION_ID);
        assertTrue(executed);
    }

    function test_sameNullifier_reverts() public {
        vm.prank(AGENT);
        constitution.proposeAction(ACTION_ID, 2);

        uint256 epoch = block.timestamp / 1 hours;
        constitution.approveAction(ACTION_ID, pA, pB, pC, _pubSignals(1, 999, epoch, ACTION_ID));

        vm.expectRevert();
        constitution.approveAction(ACTION_ID, pA, pB, pC, _pubSignals(1, 999, epoch, ACTION_ID));
    }

    function test_wrongChallenge_reverts() public {
        vm.prank(AGENT);
        constitution.proposeAction(ACTION_ID, 1);

        uint256 epoch = block.timestamp / 1 hours;
        uint256[5] memory bad = _pubSignals(1, 999, epoch, ACTION_ID + 1); // 別Action用のchallenge
        vm.expectRevert();
        constitution.approveAction(ACTION_ID, pA, pB, pC, bad);
    }

    function test_proposeAction_byNonAgent_reverts() public {
        address stranger = address(0xBEEF);
        vm.prank(stranger);
        vm.expectRevert();
        constitution.proposeAction(ACTION_ID, 1);
    }
}
