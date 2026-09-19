// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {Test} from "forge-std/Test.sol";
import {FamilyRegistry} from "../src/FamilyRegistry.sol";
import {Groth16Verifier} from "../src/Groth16Verifier.sol";

contract FamilyRegistryIntegrationTest is Test {
    FamilyRegistry registry;

    // circuits/ で snarkjs groth16 prove した本物の証明（secret=103, salt=9003, epoch=472223, challenge=777）
    uint256[2] pA = [
        0x09e800ea0f2c28c5ee83167aabe8151abba9c930847c01c014bd238f00bf3d12,
        0x1b0fb3f1eb7afbe0d47e87874f7b7e1547ec00ed69a2d1836c535f5864c0a614
    ];
    uint256[2][2] pB = [
        [0x046f32eeee36500657cf7433a071d90cc2591545045341e839e1d6ee52362c2a,
         0x27cbf722a00e0885d91023300e0f29f17a98b83fd2161c0869411ab81e0a0794],
        [0x2e01d79c2c48e842c6bf4fbc98d089c107316138429f7f56ddb09caeed37392e,
         0x13fad91c6b4776119d80eb90190d85da2b6027bfe145b22d0eaa4923b74c0922]
    ];
    uint256[2] pC = [
        0x2ce08ea5d599d8fc3ba8416a02b0863504a03ed30b1631dccd31a1856e69e12c,
        0x1874ba9e9689a340b58689409a675b163120f6cb5421f81f0fe75fdef72d6817
    ];
    uint256[5] pubSignals = [
        0x2675ee0ad166debb939d3e06c4a37b44e416eb6514d494ad11ce8e9f67364200, // root
        0x0f05085dd1850e11f9d4d1e0f7038c874950ae5305fce0b1f81a979e6249ae85, // y
        0x21928ac0ee7c4ba5023fca589fb5ea3f13ef006ef8052ed8b5fbfab7ac72ef38, // nullifier
        0x000000000000000000000000000000000000000000000000000000000007349f, // epoch = 472223
        0x0000000000000000000000000000000000000000000000000000000000000309  // challenge = 777
    ];

    function setUp() public {
        Groth16Verifier verifier = new Groth16Verifier(); // 本物のVerifier（Mockではない）
        registry = new FamilyRegistry(address(verifier), pubSignals[0]); // familyRoot = 本物のroot
        vm.warp(472223 * 1 hours); // 証明に埋め込まれたepochと現在時刻を合わせる
    }

    function test_realProof_verifiesOnChain() public {
        bool ok = registry.verifyMembership(pA, pB, pC, pubSignals);
        assertTrue(ok);
    }
}
