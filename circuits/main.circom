pragma circom 2.0.0;

include "node_modules/circomlib/circuits/poseidon.circom";
include "node_modules/circomlib/circuits/mux1.circom";

template MerkleTreeInclusionProof(nLevels) {
    signal input epoch;
    signal input challenge;
    signal input secret;
    signal input salt;

    signal input pathIndices[nLevels];
    signal input siblings[nLevels];

    signal output root;
    signal output y;
    signal output nullifier;

    component poseidons[nLevels];
    component mux[nLevels];
    signal hashes[nLevels + 1];

    component poseidon_init = Poseidon(3);
    poseidon_init.inputs[0] <== 0;
    poseidon_init.inputs[1] <== secret;
    poseidon_init.inputs[2] <== salt;
    hashes[0] <== poseidon_init.out;

    component poseidon_a1 = Poseidon(2);
    poseidon_a1.inputs[0] <== secret;
    poseidon_a1.inputs[1] <== epoch;
    signal a1;
    a1 <== poseidon_a1.out;

    component poseidon_x = Poseidon(1);
    poseidon_x.inputs[0] <== challenge;
    signal x;
    x <== poseidon_x.out;

    y <== secret + a1 * x;

    component poseidon_null = Poseidon(1);
    poseidon_null.inputs[0] <== a1;
    nullifier <== poseidon_null.out;

    for (var i = 0; i < nLevels; i++) {
        pathIndices[i] * (1 - pathIndices[i]) === 0;

        poseidons[i] = Poseidon(2);
        mux[i] = MultiMux1(2);

        mux[i].c[0][0] <== hashes[i];
        mux[i].c[0][1] <== siblings[i];

        mux[i].c[1][0] <== siblings[i];
        mux[i].c[1][1] <== hashes[i];

        mux[i].s <== pathIndices[i];

        poseidons[i].inputs[0] <== mux[i].out[0];
        poseidons[i].inputs[1] <== mux[i].out[1];

        hashes[i + 1] <== poseidons[i].out;
    }

    root <== hashes[nLevels];
}

component main {public [epoch, challenge]}= MerkleTreeInclusionProof(4);