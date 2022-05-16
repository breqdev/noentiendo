FIRST = $00
SECOND = $01

  .org $8000
reset:
  LDA #$0D; First factor (4 bits)
  STA FIRST
  LDA #$09; Second factor (4 bits)
  STA SECOND
  LDA #$00; Result (8 bits)
  LDX #$04; X register used to count the iteration

.loop:
  LSR FIRST; Shifts the first factor to the right, putting the LSB in the carry flag so we can test it
  BCC .no_add

  CLC ; Clear the carry flag (since we know its set)
  ADC SECOND; Add the second factor to the result

.no_add:
  ASL SECOND
  DEX
  BNE .loop; While X is not 0

  STA $4000

end:
  JMP end

  .org $fffa
vectors:
  .word $0000; NMI
  .word reset; RESET
  .word $0000; IRQ
