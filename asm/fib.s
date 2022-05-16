  .org $8000

reset:
  LDX #$01
  STX $00
  LDA #$02
  STA $01
loop:
  LDX $01
  ADC $00
  STA $01
  JSR print
  STX $00
  JMP loop

print:
  STA $4000
  RTS

  .org $fffa
vectors:
  .word $0000; NMI
  .word reset; RESET
  .word $0000; IRQ
