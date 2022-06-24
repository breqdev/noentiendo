  .org $F000

reset:
  LDA #$08; H
  STA $8000
  LDA #$05; E
  STA $8001
  LDA #$0C; L
  STA $8002
  STA $8003
  LDA #$0F; O
  STA $8004

  LDA #$17; W
  STA $8051
  LDA #$0F; O
  STA $8052
  LDA #$12; R
  STA $8053
  LDA #$0C; L
  STA $8054
  LDA #$04; D
  STA $8055

loop:
  JMP loop

  .org $FFFA
vectors:
  .word $0000; NMI
  .word reset; RESET
  .word $0000; IRQ

