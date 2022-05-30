PRINT_CHAR = $4003
READ_NUM = $4000

TARGET = $00

  .org $8000

reset:
  LDA #$04; target to guess
  STA TARGET;
  LDX #$04; allowed attempts
prompt_guess:
  DEX
  BEQ lose;
  JSR print_take_a_guess
  LDA READ_NUM;
  CMP TARGET;
  BMI too_low;
  BNE too_high;

  JSR print_you_win;
end:
  JMP end;


too_low:
  JSR print_too_low;
  JMP prompt_guess;
too_high:
  JSR print_too_high;
  JMP prompt_guess;

lose:
  JSR print_you_lose;
  JMP end;

print_take_a_guess:
  LDA #'G';
  STA PRINT_CHAR;
  LDA #'u';
  STA PRINT_CHAR;
  LDA #'e';
  STA PRINT_CHAR;
  LDA #'s';
  STA PRINT_CHAR;
  STA PRINT_CHAR;
  LDA #$0A; newline
  STA PRINT_CHAR;
  RTS

print_too_low:
  LDA #'L';
  STA PRINT_CHAR;
  LDA #'o';
  STA PRINT_CHAR;
  LDA #'w';
  STA PRINT_CHAR;
  LDA #$0A; newline
  STA PRINT_CHAR;
  RTS

print_too_high:
  LDA #'H';
  STA PRINT_CHAR;
  LDA #'i';
  STA PRINT_CHAR;
  LDA #'g';
  STA PRINT_CHAR;
  LDA #'h';
  STA PRINT_CHAR;
  LDA #$0A; newline
  STA PRINT_CHAR;
  RTS

print_you_win:
  LDA #'W';
  STA PRINT_CHAR;
  LDA #'i';
  STA PRINT_CHAR;
  LDA #'n';
  STA PRINT_CHAR;
  LDA #'!';
  STA PRINT_CHAR;
  LDA #$0A; newline
  STA PRINT_CHAR;
  RTS

print_you_lose:
  LDA #'L';
  STA PRINT_CHAR;
  LDA #'o';
  STA PRINT_CHAR;
  LDA #'s';
  STA PRINT_CHAR;
  LDA #'e';
  STA PRINT_CHAR;
  LDA #$0A; newline
  STA PRINT_CHAR;
  RTS

  .org $fffa
vectors:
  .word $0000; NMI
  .word reset; RESET
  .word $0000; IRQ
