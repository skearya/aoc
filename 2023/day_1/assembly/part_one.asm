; # pseudocode
; sum = 0
; first = null
; second = null
; for char in input {
; 	if char is number { 
; 		if first == null {
; 			first = char
; 		}

; 		second = char
; 	} else if char == newline {
; 		sum += first * 10 + second
		
; 		first = null
; 		second = null
; 	}
; }

section .data
    input: incbin "../input.txt"
    input_len: equ $-input

section .text
   global _start

_start:
    ;/ mul requires a register as an operand
    mov r10, 10
    ; file byte index
    mov rcx, 0

    ; sum
    mov rdi, 0
    ; first digit
    mov r8, 0
    ; second digit
    mov r9, 0

byte_loop:
    ; current character (dl is byte end of rdx)
    mov dl, [input + rcx]
    
    cmp rdx, `\n`
    jne not_new_line

    ; convert from char to int (see ascii table)
    sub r8, '0'
    sub r9, '0'

    mov rax, r8
    ; r10 = 10
    mul r10

    add rdi, rax
    add rdi, r9

    mov r8, 0
    mov r9, 0

    jmp continue
not_new_line:
    cmp rdx, '0'
    jl continue

    cmp rdx, '9'
    jg continue

    cmp r8, 0
    jnz first_not_null

    mov r8, rdx
first_not_null:
    mov r9, rdx
continue:
    inc rcx
    cmp rcx, input_len
    jne byte_loop

exit:
    mov rax, 60
    mov rdi, 0
    syscall
