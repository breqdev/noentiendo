FIRST = $00
SECOND = $01
RESULT = $02

ITERATION = $03

  .org $8000
reset:
  LDA #$0D
  STA FIRST
  LDA #$09
  STA SECOND
  LDA #$00
  STA RESULT
  LDA #$00
  STA ITERATION

.loop:
  LDA #$01
  BIT FIRST
  BEQ .no_add

  LDA RESULT
  ADC SECOND
  STA RESULT

.no_add:
  LSR FIRST
  ASL SECOND
  INC ITERATION

  SEC
  LDA ITERATION
  CMP #$04
  BNE .loop

  LDA RESULT
  STA $4000

end:
  JMP end

  .org $fffa
vectors:
  .word $0000; NMI
  .word reset; RESET
  .word $0000; IRQ
