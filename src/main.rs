fn main() {
    // worry about CLI later. For now, develop algo.

    // test inputs. Plaintext is unicode
    let key=        [0x9f, 0xb6, 0x78, 0xeb, 0xbb, 0x43, 0x9e, 0x76, 0xd9, 0x97, 0xff, 0x2b, 0xac, 0x85, 0x03, 0x23, 
                                0xbf, 0x2e, 0x1c, 0x87, 0xd0, 0x67, 0x03, 0x5f, 0x31, 0x17, 0xd4, 0x75, 0x36, 0xcf, 0x17, 0x70];
    let plaintext = [0x54, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x65, 0x6E, 0x63, 0x72, 
                                0x79, 0x70, 0x74, 0x69, 0x6F, 0x6E, 0x20, 0x6D, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x2E, 0x20, 
                                0x49, 0x74, 0x20, 0x77, 0x69, 0x6C, 0x6C, 0x20, 0x70, 0x72, 0x6F, 0x62, 0x61, 0x62, 0x6C, 0x79, 
                                0x20, 0x72, 0x65, 0x71, 0x75, 0x69];
}


fn add_round_key() {

}

fn sub_bytes() {
    
}

fn shift_rows() {
    
}

fn mix_columns() {
    
}



/*

AES encryption/decryption tool.

USAGE:
    aes <COMMAND> [OPTIONS]

COMMANDS:
    encrypt     Encrypt input to output
    decrypt     Decrypt input to output
    help        Print this message

OPTIONS (COMMON):
    -i, --input <PATH>      Input file path. Use '-' to read from stdin
    -o, --output <PATH>     Output file path. Use '-' to write to stdout
	-k, --key <PATH>        Key file path. 
                            Use '-' for stdin/stdout (inferred from COMMAND)
                            Additional options exist for generation (see below)

KEY GENERATION OPTIONS (ENCRYPTION ONLY):
	--generate-key          Generate a random key
                            Will be written to PATH specified after -k or --key
	--key-size <BITS>       Only valid with --generate-key
                            Generate a key of size BITS
							BITS can be 128, 192, or 256. Default: 256
*/