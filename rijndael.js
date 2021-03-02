var RCONi, roundKeys;

function AES_encrypt(plaintext) {
    // Whiten:
    var state = AddRoundKey(plaintext, roundKeys[0]); // state is in hexadecimal form

    /* MAIN ROUNDS */
    for (let j=1; j<=9; j++) {
        // SubBytes:
        var SubBytes = [];
        for (let i=0; i<state.length; i++) {
            var internalArray = [];
            for (let n=0; n<4; n++) internalArray.push(sbox[parseInt(state[i][n], 16)]);
            SubBytes.push(internalArray);
        }

        state = byteToHex(SubBytes); // state is in hexadecimal form
        
        // ShiftRows:
        state = [[state[0][0], state[0][1], state[0][2], state[0][3]],
                 [state[1][1], state[1][2], state[1][3], state[1][0]],
                 [state[2][2], state[2][3], state[2][0], state[2][1]],
                 [state[3][3], state[3][0], state[3][1], state[3][2]]];

        // MixColumns:
        state = MixColumns(state); // state is in binary form
        
        // AddRoundKey:
        state = AddRoundKey(state, roundKeys[j]); // state is in hexadecimal form
    }

    SubBytes = [];
    for (let i=0; i<state.length; i++) {
        var internalArray = [];
        for (let n=0; n<4; n++) internalArray.push(sbox[parseInt(state[i][n], 16)]);
        SubBytes.push(internalArray);
    }
    
    /* FINAL ROUND */
    // SubBytes
    state = byteToHex(SubBytes); // state is in hexadecimal form

    // ShiftRows:
    state = [[state[0][0], state[0][1], state[0][2], state[0][3]],
             [state[1][1], state[1][2], state[1][3], state[1][0]],
             [state[2][2], state[2][3], state[2][0], state[2][1]],
             [state[3][3], state[3][0], state[3][1], state[3][2]]];
    state = byteToBinary(hexToByte(state)); // state is now binary
    
    // AddRoundKey:
    return AddRoundKey(state, roundKeys[10]); // return ciphertext in hexadecimal form.
}


function AES_decrypt(ciphertext) {
    // Inverse Whiten:
    var state = AddRoundKey(ciphertext, roundKeys[10])

    // Inverse ShiftRows:
    state = [[state[0][0], state[0][1], state[0][2], state[0][3]],
             [state[1][3], state[1][0], state[1][1], state[1][2]],
             [state[2][2], state[2][3], state[2][0], state[2][1]],
             [state[3][1], state[3][2], state[3][3], state[3][0]]];

    // Inverse SubBytes:
    var SubBytes = [];
    for (let i=0; i<state.length; i++) {
        var internalArray = [];
        for (let n=0; n<4; n++) internalArray.push(sboxInv[parseInt(state[i][n], 16)]);
        SubBytes.push(internalArray);
    }
    state = byteToBinary(SubBytes); // convert to binary for AddRoundKey

    // Begin rounds in reverse order:
    for (let j=9; j>=1; j--) {
        state = AddRoundKey(state, roundKeys[j]); // state is now hexadecimal

        // Inverse MixColumns:
        state = MixColumnsInv(state); // state is now binary
        state = binToHex(state); // state is now hexadecimal

        // Inverse ShiftRows:
        state = [[state[0][0], state[0][1], state[0][2], state[0][3]],
                 [state[1][3], state[1][0], state[1][1], state[1][2]],
                 [state[2][2], state[2][3], state[2][0], state[2][1]],
                 [state[3][1], state[3][2], state[3][3], state[3][0]]];

        // Inverse SubBytes:
        SubBytes = [];
        for (let i=0; i<state.length; i++) {
            var internalArray = [];
            for (let n=0; n<4; n++) internalArray.push(sboxInv[parseInt(state[i][n], 16)]);
            SubBytes.push(internalArray);
        }        
        state = byteToBinary(SubBytes); // state is now binary
    }

    return AddRoundKey(state, roundKeys[0]); // return plaintext in hexadecimal form.
}

function KeyExpansionCore(bytes) {
    // rotate word:
    var RotWord = [];
    RotWord[0] = bytes[1][3];
    RotWord[1] = bytes[2][3];
    RotWord[2] = bytes[3][3];
    RotWord[3] = bytes[0][3];

    // subword using sbox
    var SubWord = [];
    for (let k=0; k<4; k++) {
        SubWord[k] = parseInt(RotWord[k], 2); // convert to hexadecimal
        SubWord[k] = sbox[SubWord[k]]; // sub sbox
        SubWord[k] = parseInt(SubWord[k]).toString(2).padStart(8, "0");  
    }
    var col1 = [bytes[0][0], bytes[1][0], bytes[2][0], bytes[3][0]]; // first column of roundKeys[i-1]

    var postXor = xorWords(SubWord, col1); // xor first column of roundKeys[i-1] with subbed word

    // lookup rcon[RCONi] and convert to binary:
    var rconVal = parseInt(rcon[RCONi]).toString(2).padStart(8, "0"); 
    // xor with postXor[0]:
    var xorRCON = "";
    for (let k=0; k<8; k++) xorRCON += (postXor[0][k] ^ rconVal[k]);
    postXor[0] = xorRCON;

    return postXor;
}


function KeyExpansion(key) {
    roundKeys = [key]; // roundKeys[0] is simply the key

    // loop 10 times with round constant iteration (RCONi) beginning 1
    for (RCONi=1; RCONi<=10; RCONi++) {
        var TempCol = []; // TempCol[0] is first column of roundKeys[i]
        var prvR = roundKeys[RCONi-1]; // store previous roundKey
        TempCol[0] = KeyExpansionCore(prvR);
        TempCol[1] = xorWords(TempCol[0], [prvR[0][1], prvR[1][1], prvR[2][1], prvR[3][1]]);
        TempCol[2] = xorWords(TempCol[1], [prvR[0][2], prvR[1][2], prvR[2][2], prvR[3][2]]);
        TempCol[3] = xorWords(TempCol[2], [prvR[0][3], prvR[1][3], prvR[2][3], prvR[3][3]]);

        // format roundKeys[i]:
        var currRoundKey = [];
        for (let i=0; i<4; i++) {
            currRoundKey.push([TempCol[0][i], TempCol[1][i], TempCol[2][i], TempCol[3][i]]); 
        }
        roundKeys.push(currRoundKey);
    }
}

function AddRoundKey(state, key) {
    // takes state and key in binary form and outputs in hexadecimal form.
    return binToHex(xorBin(state, key));
}

function MixColumns(inpArray) {
    // takes hex and outputs binary
    var mixedCols = [];
    var temp = [];

    for (let i=0; i<4; i++) {
        temp.push([
            xorSngl(xorSngl(xorSngl(mul2fn(inpArray[0][i]), mul3fn(inpArray[1][i])), hexToBin(inpArray[2][i])), hexToBin(inpArray[3][i])),
            xorSngl(xorSngl(xorSngl(hexToBin(inpArray[0][i]), mul2fn(inpArray[1][i])), mul3fn(inpArray[2][i])), hexToBin(inpArray[3][i])),
            xorSngl(xorSngl(xorSngl(hexToBin(inpArray[0][i]), hexToBin(inpArray[1][i])), mul2fn(inpArray[2][i])), mul3fn(inpArray[3][i])),
            xorSngl(xorSngl(xorSngl(mul3fn(inpArray[0][i]), hexToBin(inpArray[1][i])), hexToBin(inpArray[2][i])), mul2fn(inpArray[3][i])),
        ]); // Galois field arithmetic.
    }

    for (let i=0; i<4; i++) {
        mixedCols.push([temp[0][i], temp[1][i], temp[2][i], temp[3][i]]); // correct column/row order
    }

    return mixedCols;
}

function MixColumnsInv(inpArray) {
    // takes hex and outputs binary
    var mixedCols = [];
    var temp = [];

    for (let i=0; i<4; i++) {
        temp.push([
            xorSngl(xorSngl(xorSngl(mul14fn(inpArray[0][i]), mul11fn(inpArray[1][i])), mul13fn(inpArray[2][i])), mul9fn(inpArray[3][i])),
            xorSngl(xorSngl(xorSngl(mul9fn(inpArray[0][i]), mul14fn(inpArray[1][i])), mul11fn(inpArray[2][i])), mul13fn(inpArray[3][i])),
            xorSngl(xorSngl(xorSngl(mul13fn(inpArray[0][i]), mul9fn(inpArray[1][i])), mul14fn(inpArray[2][i])), mul11fn(inpArray[3][i])),
            xorSngl(xorSngl(xorSngl(mul11fn(inpArray[0][i]), mul13fn(inpArray[1][i])), mul9fn(inpArray[2][i])), mul14fn(inpArray[3][i])),
        ]); // Inverse Galois field arithmetic.
    }

    for (let i=0; i<4; i++) {
        mixedCols.push([temp[0][i], temp[1][i], temp[2][i], temp[3][i]]); // correct column/row order
    }

    return mixedCols;
}