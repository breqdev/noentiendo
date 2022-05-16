FIRST = $00
SECOND = $01
RESULT = $02

  .org $8000
reset:
  LDA #$0D; First factor (4 bits)
  STA FIRST
  LDA #$09; Second factor (4 bits)
  STA SECOND
  LDA #$00; Initialize the result to 0
  STA RESULT
  LDX #$04; X register used to count the iteration

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
  DEX
  BNE .loop; While X is not 0

  LDA RESULT
  STA $4000

end:
  JMP end

  .org $fffa
vectors:
  .word $0000; NMI
  .word reset; RESET
  .word $0000; IRQ
