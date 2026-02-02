# AES encryption/decryption tool.
## Usage
```
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
```
