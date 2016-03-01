# IPSter

IPS ([International Patching Format](http://www.zerosoft.zophar.net/ips.php)) patcher (and differ)

## Checklist
 - [ ] apply a patch file to an original file (output to a third file)
 - [X] create a .IPS (diff) file
 - [ ] handle corner case where a patch chunk begins at 0x454F46 (IPS format requires a IPS file to end with "EOF", i.e. "45 4F 46")
 - [ ] handle files bigger than 2^24 bytes (i.e. refuse to work on them, since addressing is limited to 24bits)
 - [ ] handle patch chunks bigger than 2^16 bytes(i.e. split them in smaller chunks of max 0xFFFF bytes)
 - [ ] add RLE support
