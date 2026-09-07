pragma circom 2.0.0;

include "circomlib/circuits/poseidon.circom";
include "circomlib/circuits/switcher.circom";

template MerkleTreeCheckr(levels){
    signal input leaf;
    signal input pathElements[levels];
    signal output root;

    component h = poseidon(2);
    h.inputs[0] <== a;
    h.inputs[1] <== b;

    for (var i = 0; i < levels; i++){
        a <== leaf;
        b <== pathElements[i];
        let temp = h.out;
        leaf <== temp;
    }  
}

component main(public [root]) = MerkleTreeCheckr(4);
