NUM_A = $00
NUM_B = $01

  .org $8000

reset:
  LDX #$01
  STX NUM_A
  LDA #$02
  STA NUM_B
loop:
  LDX NUM_B
  ADC NUM_A
  STA NUM_B
  JSR print
  STX NUM_A
  JMP loop

print:
  STA $4000
  RTS

  .org $fffa
vectors:
  .word $0000; NMI
  .word reset; RESET
  .word $0000; IRQ
