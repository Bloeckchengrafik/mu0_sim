loop:
    LDA Total
addinstr:
    ADD Table
    STO Total
    LDA addinstr
    ADD One
    STO addinstr
    LDA Count
    SUB One
    STO Count
    JGE loop
    STP

Total:
    DEFW 0
One:
    DEFW 1
Count:
    DEFW 4
Table:
    DEFW 39
    DEFW 25
    DEFW 4
    DEFW 98
    DEFW 17
