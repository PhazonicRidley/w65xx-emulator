import csv
s = """ADC add with carry
AND and (with accumulator)
ASL arithmetic shift left
BCC branch on carry clear
BCS branch on carry set
BEQ branch on equal (zero set)
BIT bit test
BMI branch on minus (negative set)
BNE branch on not equal (zero clear)
BPL branch on plus (negative clear)
BRK break / interrupt
BVC branch on overflow clear
BVS branch on overflow set
CLC clear carry
CLD clear decimal
CLI clear interrupt disable
CLV clear overflow
CMP compare (with accumulator)
CPX compare with X
CPY compare with Y
DEC decrement
DEX decrement X
DEY decrement Y
EOR exclusive or (with accumulator)
INC increment
INX increment X
INY increment Y
JMP jump
JSR jump subroutine
LDA load accumulator
LDX load X
LDY load Y
LSR logical shift right
NOP no operation
ORA or with accumulator
PHA push accumulator
PHP push processor status (SR)
PLA pull accumulator
PLP pull processor status (SR)
ROL rotate left
ROR rotate right
RTI return from interrupt
RTS return from subroutine
SBC subtract with carry
SEC set carry
SED set decimal
SEI set interrupt disable
STA store accumulator
STX store X
STY store Y
TAX transfer accumulator to X
TAY transfer accumulator to Y
TSX transfer stack pointer to X
TXA transfer X to accumulator
TXS transfer X to stack pointer
TYA transfer Y to accumulator"""


s = s.strip().split("\n")
d = dict(map(lambda string: (string[:3], string[3:].strip()), s))
with open('instructions.csv', 'w', newline='') as csvfile:
    field_names = ['Instruction', 'Description']
    writer = csv.DictWriter(csvfile, fieldnames=field_names)
    writer.writeheader()
    for k, v in d.items():
        writer.writerow({'Instruction': k, 'Description': v})
