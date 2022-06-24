  .org $8000
reset:
  LDA #$01
  STA $4000
  LDA #$05
  STA $4001
  LDA #$08
  STA $43ff
loop:
  JMP loop

  .org $fffa
vectors:
  .word $0000; NMI
  .word reset; RESET
  .word $0000; IRQ
