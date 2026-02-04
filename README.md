Super projet rust
Features actuellement implémentées:
Obfuscation:
  - Strings chiffrés (obfstr)
  - Control Flow Flattening (goldberg)
Packing:
  - Pas de compression
  - Chiffrement (seuleemnt un xor pour le moment)
  - Execution dynamique en mémoire (RunPE)
Process Hollowing (windows et x64 seulement)
Anti-Debug:
  - IsDebuggerPresent()
  - Timing
  - Corruption par le déchiffrement avec une clé partielle si quelque chose est anormal

Detecte si l'exe est un PE, elf ou juste du shellcode
