# Hack ASM Preprocessor

A utility which adds function calling and return support to Hack ASM

## Description

Currently this preprocessor supports two instructions `#call <label>` and `#ret`.  The #call instruction will store the current address and jump to the provided label.  The #ret instruction will retrieve the stored address and jump back to it.  It is capable of storing multiple addresses at once - the upper limit is 16382 deep or whenever you overwrite the stack.  Input can be provided from stdin or as a filename as the first argument.

## Usage

```
#CALL TEST1

@END
(END)
0;JMP

(TEST1)
@5
D=A
@0
M=D
#RET
```

This is pretty much the minimal functioning use scenario.  It will immediately jump to the function TEST1 - which will set @0 to 5 - and then it will jump back to right after #CALL TEST1.  It produces the ASM below.

```
@16383
D=A-1
M=D
@0
// JUMPING TO LABEL TEST1
// STORE CURRENT ADDRESS
D=A
@16383
A=M
M=D
@16383
M=M-1
// STORED
@TEST1
0;JMP

@END
(END)
0;JMP

(TEST1)
@5
D=A
@0
M=D
// RETURN FROM STORED ADDRESS
@16383
M=M+1
@16383
A=M
A=M
D=A
@12
A=D+A
0;JMP
// RETURNED
```