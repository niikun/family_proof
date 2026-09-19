// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {Test} from "forge-std/Test.sol";
import {FamilyRegistry} from "../src/FamilyRegistry.sol";
import {IGroth16Verifier} from "../src/IGroth16Verifier.sol";

// 常に true を返すダミーの Verifier
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

contract FamilyRegistryTest is Test {
    FamilyRegistry registry;
    uint256 constant ROOT = 111;

    uint256[2] pA;
    uint256[2][2] pB;
    uint256[2] pC;

    function setUp() public {
        MockVerifier verifier = new MockVerifier();
        registry = new FamilyRegistry(address(verifier), ROOT);
    }

    function _pubSignals(uint256 y, uint256 nullifier, uint256 epoch, uint256 challenge)
        internal pure returns (uint256[5] memory)
    {
        return [ROOT, y, nullifier, epoch, challenge];
    }

    function test_firstProof_succeeds() public {
        uint256 epoch = block.timestamp / 1 hours;
        bool ok = registry.verifyMembership(pA, pB, pC, _pubSignals(1, 999, epoch, 111));
        assertTrue(ok);
    }

    function test_replaySameChallenge_reverts() public {
        uint256 epoch = block.timestamp / 1 hours;
        registry.verifyMembership(pA, pB, pC, _pubSignals(1, 999, epoch, 111));
        vm.expectRevert();
        registry.verifyMembership(pA, pB, pC, _pubSignals(1, 999, epoch, 111));
    }

    function test_differentChallenge_emitsPotentialLeak() public {
        uint256 epoch = block.timestamp / 1 hours;
        registry.verifyMembership(pA, pB, pC, _pubSignals(1, 999, epoch, 111));

        vm.expectEmit(true, false, false, true);
        emit FamilyRegistry.PotentialLeak(999, 111, 1, 222, 2, epoch);
        registry.verifyMembership(pA, pB, pC, _pubSignals(2, 999, epoch, 222));
    }

    function test_wrongRoot_reverts() public {
        uint256 epoch = block.timestamp / 1 hours;
        uint256[5] memory bad = [ROOT + 1, uint256(1), uint256(999), epoch, uint256(111)];
        vm.expectRevert();
        registry.verifyMembership(pA, pB, pC, bad);
    }

    function test_staleEpoch_reverts() public {
        uint256 wrongEpoch = block.timestamp / 1 hours + 999999; // 確実に現在と一致しない値
        vm.expectRevert();
        registry.verifyMembership(pA, pB, pC, _pubSignals(1, 999, wrongEpoch, 111));
    }

    function test_updateRoot_byOwner_succeeds() public {
    uint256 newRoot = 222;

    vm.expectEmit(true, false, false, true);
    emit FamilyRegistry.RootUpdated(newRoot);
    registry.updateRoot(newRoot);

    assertEq(registry.familyRoot(), newRoot);
    }

    function test_updateRoot_byNonOwner_reverts() public {
        address stranger = address(0xBEEF);
        vm.prank(stranger);
        vm.expectRevert();
        registry.updateRoot(999);
    }
}
