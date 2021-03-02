var internalArray;

function hexOutput(inp) {
    var output = ""; // takes single array of hexadecimal values and converts it to hexadecimal string
    for (let i=0; i<inp.length; i++) output += inp[i] + " ";
    return output;
}

function binOutput(inp) {
    var output = ""; // takes single array of hexadecimal values and converts it to binary string
    for (let i=0; i<inp.length; i++) output += (parseInt(inp[i], 16)).toString(2).padStart(8, '0') + " ";
    return output;
}

function xorBin(bin1, bin2) {
    var xor = []; // XORs two 4x4 binary arrays
    for (let i=0; i<4; i++) {
        var internalArray = [];
        for (let n=0; n<4; n++) {
            internalXor="";
            for (let m=0; m<bin1[i][n].length; m++) internalXor += (bin1[i][n][m] ^ bin2[i][n][m]);
            internalArray.push(internalXor);
        }
        xor.push(internalArray)
    }

    return xor;
}

function xorSngl(bin1, bin2) {
    var xored = ""; // XORs two single binary strings
    for (let i=0; i<8; i++) xored += (bin1[i] ^ bin2[i]);
    return xored;
}


function splitASCII(inputStr) {
    // splits string of characters into an array of 4x4 arrays
    var toCrypt = [];
    toCrypt[0] = stringToBinary(inputStr.substring(0, 16));
    for (let i=1; inputStr.length > 16; i++) {
        inputStr = inputStr.slice(16, inputStr.length);
        toCrypt[i] = stringToBinary(inputStr.substring(0, 16));
    }
    return toCrypt;
}

function splitHex(inputStr) {
    // splits string of hexadecimals into an array of 4x4 arrays
    inputStr = inputStr.split(" "); // turn the characters into an array
    if (inputStr[inputStr.length-1].length < 2) inputStr.splice(inputStr.length-1, 1); // the final input is not a hexadecimal value (it's a space, a full stop, etc), so cut it     
    var toCrypt = []; // final collection of 4x4 arrays
    for (let p=0; p<inputStr.length; p+=16) {
        // parent loop. Repeat for p is less than the number of items in inputStr, adding 16 to p
        output = []; // 4x4 array
        for (let i=0; i<4; i++) {
            internalArray = []; // one row of a 4x4 array
            for (let n=0; n<=12; n+=4) {
                if (inputStr[p+i+n]==null) internalArray.push("00000000"); // desired value does not exist, so pad using the null character. 
                else internalArray.push(hexToBin(inputStr[p+i+n])); // push character to its corresponding position.
            }
            output.push(internalArray);
        }
        toCrypt.push(output);
    }
    return toCrypt;
}


function splitBin(inputStr) {
    // splits string of binary into an array of 4x4 arrays
    inputStr = inputStr.split(" ");
    if (inputStr[inputStr.length-1].length < 8) inputStr.splice(inputStr.length-1, 1);
    var loopLength = Math.ceil(inputStr.length/16);
    var toCrypt = [];
    for (let p=0; p<inputStr.length; p+=16) {
        output = []
        for (let i=0; i<4; i++) {
            internalArray = [];
            for (let n=0; n<=12; n+=4) {
                if (inputStr[p+i+n]==null) internalArray.push("00000000");
                else internalArray.push(inputStr[p+i+n]);
            }
            output.push(internalArray);
        }
        toCrypt.push(output);
    }
    
    return toCrypt;
}

function keyFormat(key, keyType) {
    key = key.split(" "); // splits key into 4x4 array
    if (keyType.includes("Hex")) for (let i=0; i<key.length; i++) key[i] = hexToBin(key[i]);
    var output = [];
    for (let i=0; i<4; i++) {
        internalArray = [];
        for (let n=0; n<=12; n+=4) {
            if (key[i+n]==null) internalArray.push("00000000");
            else internalArray.push(key[i+n]);
        }
        output.push(internalArray);
    }
    return output;
}


/* CONVERSIONS */
function stringToBinary(str) {
    var output = []; // converts 16 byte string to 4x4 array
    for (let i=0; i<4; i++) {
        internalArray = [];
        for (let n=0; n<=12; n+=4) {
            if (str[i+n]==null) internalArray.push("00000000"); // hex 0x00
            else internalArray.push((str.charCodeAt(i+n)).toString(2).padStart(8, '0'));
        }
        output.push(internalArray)
    }
    return output;
}


function byteToHex(byte) {
    var hex = []; // convert 4x4 ASCII char codes array to hexadecimal
    for (let i=0; i<4; i++) {
        var internalArray = [];
        for (let n=0; n<4; n++) internalArray.push(byte[i][n].toString(16).padStart(2, "0"));
        hex.push(internalArray);
    }
    return hex;
}

function hexToByte(hex) {
    var byte = []; // convert 4x4 hexadecimal array to ASCII char codes
    for (let i=0; i<4; i++) {
        var internalArray = [];
        for (let n=0; n<4; n++) {
            internalArray.push(parseInt(hex[i][n], 16));
        }
        byte.push(internalArray);
    }
    return byte;
}

function binaryToByte(bin) {
    var output = []; // convert 4x4 binary array to ASCII char codes
    for (let i=0; i<4; i++) {
        var internalArray = [];
        for (let n=0; n<4; n++) {
            internalArray.push(parseInt(bin[i][n], 2));
        }
        output.push(internalArray);
    }
    return output;
}

function byteToBinary(byte) {
    var output = []; // convert 4x4 ASCII char codes array to binary
    for (let i=0; i<4; i++) {
        var internalArray = [];
        for (let n=0; n<4; n++) internalArray.push((byte[i][n].toString(2)).padStart(8, '0'));
        output.push(internalArray)
    }
    return output;
}

function hexToBin(hex) {
    return (parseInt(hex, 16)).toString(2).padStart(8, '0'); // convert single hexadecimal value to binary
}

function binToHex(bin) {
    var output = []; // convert 4x4 binary array to hexadecimal
    for (let i=0; i<4; i++) {
        var internalArray = [];
        for (let n=0; n<4; n++) internalArray.push(parseInt(bin[i][n], 2).toString(16).padStart(2, "0"));
        output.push(internalArray)
    }
    return output;
}

function xorWords(bin1, bin2) {
    var xored = []; // XORs two arrays of 4 string values
    for (let i=0; i<4; i++) {
        var temp = "";
        for (let n=0; n<8; n++) temp += bin1[i][n] ^ bin2[i][n];
        xored.push(temp);
    }
    return xored;
}

function arrayToString(arr) {
    var output = []; // takes 4x4 hexadecimal array and converts it to a single array of hexadecimal values
    for (let i=0; i<arr.length; i++) {
        hx = arr[i];
        for (let n=0; n<4; n++) output.push(hx[0][n], hx[1][n], hx[2][n], hx[3][n]);
    }
    return output;
}

function hexToString(arr) {
    var output = ""; // takes array of hexadecimal values and converts it to a string
    for (let i=0; i<arr.length; i++) output += String.fromCharCode(parseInt(arr[i], 16))
    return output;
}



/* TABLE LOOKUPS */
function mul2fn(hex) {
    hex = parseInt(hex, 16);
    var mult = (mul2[hex]).toString(16);
    return hexToBin(mult);
}

function mul3fn(hex) {
    hex = parseInt(hex, 16);
    var mult = (mul3[hex]).toString(16);
    return hexToBin(mult);
}

function mul9fn(hex) {
    hex = parseInt(hex, 16);
    var mult = (mul9[hex]).toString(16);
    return hexToBin(mult);
}

function mul11fn(hex) {
    hex = parseInt(hex, 16);
    var mult = (mul11[hex]).toString(16);
    return hexToBin(mult);
}

function mul13fn(hex) {
    hex = parseInt(hex, 16);
    var mult = (mul13[hex]).toString(16);
    return hexToBin(mult);
}

function mul14fn(hex) {
    hex = parseInt(hex, 16);
    var mult = (mul14[hex]).toString(16);
    return hexToBin(mult);
}