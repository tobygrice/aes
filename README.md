# AES encryption/decryption tool.
## Usage
```
USAGE:
    aes <COMMAND> [OPTIONS]

COMMANDS:
    encrypt     Encrypt input to output
    decrypt     Decrypt input to output
    help        Print help message

OPTIONS (COMMON):
    -i, --input <PATH>      Input file path. Use '-' to read from stdin
    -o, --output <PATH>     Output file path. Use '-' to write to stdout
	-k, --key <PATH>        Key file path. 
                            Use '-' for stdin/stdout (inferred from COMMAND)
                            Additional options exist for generation (see below)

KEY GENERATION OPTIONS (ENCRYPTION ONLY):
	--generate-key          Generate a random 256 bit key
                            Will be written to PATH specified after -k / --key
```
