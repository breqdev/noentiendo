MAPPED_STDIO = $4001

  .org $8000

reset:
  LDA #$32
  BRK
  .byte $20
loop:
  JMP loop

irq:
  ; Push registers
  PHA
  TXA
  PHA
  TYA
  PHA

  ; Read the argument to BRK
  ; The address to jump to after the interrupt is pushed on the stack,
  ; and the BRK will be immediately before that.

  TSX ; Transfer stack pointer to X

  INX ; look behind the Y, X, and A registers
  INX
  INX

  INX ; high byte
  LDA $0100,X
  STA $01

  INX ; low byte
  LDA $0100,X
  SEC
  SBC #$01; Subtract 1 to look at the byte *before* where we will jump back
  STA $00


  LDY #$00; clear Y so we can LDA indirect
  LDA ($00),Y; Load from that address

  CMP #$20
  BEQ syscall_20
  CMP #$21
  BEQ syscall_21
  CMP #$22
  BEQ syscall_22

  JMP irq_return

syscall_20:
  ; Print the A register previous contents to stdout
  DEX
  DEX
  DEX

  LDA $0100,X
  STA $4002
  JMP irq_return

syscall_21:
  ; Read a byte from stdin and store it in the A register
  DEX
  DEX
  DEX

  LDA $4002
  STA $0100,X
  JMP irq_return

syscall_22:
  JMP irq_return


irq_return:
  ; Pull registers
  PLA
  TAY
  PLA
  TAX
  PLA

  RTI

  .org $fffa
vectors:
  .word $0000; NMI
  .word reset; RESET
  .word irq; IRQ
