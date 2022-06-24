  .org $8000
reset:
  LDA #$01
  STA $0200
  LDA #$05
  STA $0201
  LDA #$08
  STA $0202
loop:
  JMP loop

  .org $fffa
vectors:
  .word $0000; NMI
  .word reset; RESET
  .word $0000; IRQ
