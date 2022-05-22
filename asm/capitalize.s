MAPPED_STDIO = $4001

  .org $8000

reset:
  LDA MAPPED_STDIO
  CMP #$61
  BMI skip_capitalize
  CMP #$7B
  BPL skip_capitalize

  AND #$DF

skip_capitalize:
  STA MAPPED_STDIO
  JMP reset

  .org $fffa
vectors:
  .word $0000; NMI
  .word reset; RESET
  .word $0000; IRQ
