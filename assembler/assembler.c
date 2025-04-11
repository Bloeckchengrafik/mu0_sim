#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef enum {
  LDA = 0,
  STO,
  ADD,
  SUB,
  JMP,
  JGE,
  JNE,
  STP,
  DEFW
} InstructionType;

typedef struct {
  char *label;
  InstructionType type;
  char *arg;
} Instruction_t;

typedef struct InstructionListNode InstructionListNode_t;
struct InstructionListNode {
  Instruction_t *instruction;
  InstructionListNode_t *next;
};

void instruction_append(InstructionListNode_t **list, char *label,
                        InstructionType type, char *arg) {
  Instruction_t *new_instruction = malloc(sizeof(Instruction_t));

  if (label) {
    new_instruction->label = strdup(label);
  } else {
    new_instruction->label = NULL;
  }

  new_instruction->type = type;

  if (arg) {
    new_instruction->arg = strdup(arg);
  } else {
    new_instruction->arg = NULL;
  }

  InstructionListNode_t *new_node = malloc(sizeof(InstructionListNode_t));
  new_node->instruction = new_instruction;
  new_node->next = NULL;

  if (*list == NULL) {
    *list = new_node;
    return;
  }

  InstructionListNode_t *current = *list;
  while (current->next != NULL) {
    current = current->next;
  }
  current->next = new_node;
}

int instruction_idx_find(InstructionListNode_t *ptr, char *label) {
  int index = 0;

  while (ptr != NULL) {
    if (ptr->instruction->label != NULL &&
        strcmp(ptr->instruction->label, label) == 0) {
      return index;
    }
    ptr = ptr->next;
    index++;
  }

  return -1;
}

char is_whitespace(char s) {
  return s == ' ' || s == '\t' || s == '\n' || s == '\r';
}

void strip(char **start, size_t *len) {
  char *s = *start;
  int length = *len;

  while (length > 0 && is_whitespace(*s)) {
    s++;
    length--;
  }

  while (length > 0 && is_whitespace(s[length - 1])) {
    length--;
  }

  *start = s;
  *len = length;
}

void parse_instructions(FILE *src, InstructionListNode_t **list) {
  char *line = NULL;
  ssize_t read;
  size_t len = 0;
  char *context_label = NULL;
  while ((read = getline(&line, &len, src)) != -1) {
    char *myline = malloc(read);
    memcpy(myline, line, read);
    size_t mylen = read;
    strip(&myline, &mylen);
    myline[mylen] = '\0';

    if (mylen == 0) {
      continue;
    }

    if (myline[mylen - 1] == ':') {
      context_label = malloc(mylen - 1);
      memcpy(context_label, myline, mylen - 1);
    } else {
      if (memcmp(myline, "LDA ", 4) == 0) {
        instruction_append(list, context_label, LDA, myline + 4);
        context_label = NULL;
      } else if (memcmp(myline, "STO ", 4) == 0) {
        instruction_append(list, context_label, STO, myline + 4);
        context_label = NULL;
      } else if (memcmp(myline, "ADD ", 4) == 0) {
        instruction_append(list, context_label, ADD, myline + 4);
        context_label = NULL;
      } else if (memcmp(myline, "SUB ", 4) == 0) {
        instruction_append(list, context_label, SUB, myline + 4);
        context_label = NULL;
      } else if (memcmp(myline, "JGE ", 4) == 0) {
        instruction_append(list, context_label, JGE, myline + 4);
        context_label = NULL;
      } else if (memcmp(myline, "JNE ", 4) == 0) {
        instruction_append(list, context_label, JNE, myline + 4);
        context_label = NULL;
      } else if (memcmp(myline, "JMP ", 4) == 0) {
        instruction_append(list, context_label, JMP, myline + 4);
        context_label = NULL;
      } else if (memcmp(myline, "STP", 3) == 0) {
        instruction_append(list, context_label, STP, NULL);
        context_label = NULL;
      } else if (memcmp(myline, "DEFW ", 5) == 0) {
        instruction_append(list, context_label, DEFW, myline + 5);
        context_label = NULL;
      } else {
        printf("invalid line: %s\n", myline);
        exit(1);
      }
    }
  }

  free(line);
}

uint16_t parse_i16(char *string) {
  if (string == NULL)
    return 0;
  int value = 0;
  if (string[0] == '0' && string[1] == 'x') {
    sscanf(string + 2, "%x", &value);
  } else {
    value = atoi(string);
  }
  return (uint16_t)value;
}

uint16_t compile_instruction(InstructionListNode_t *list, Instruction_t *curr) {
  if (curr->type == DEFW) {
    return parse_i16(curr->arg);
  }
  if (curr->type == STP) {
    return STP << 12;
  }
  int idx = instruction_idx_find(list, curr->arg);
  if (idx == -1) {
    printf("label %s not found \n", curr->arg);
    exit(1);
  }
  return (((uint16_t)curr->type) << 12) | ((uint16_t)idx);
}

void write_symbols(InstructionListNode_t *list, const char *bin_filename) {
    // Create the symbols filename by appending ".symbols" to the binary filename
    char *symbols_filename = malloc(strlen(bin_filename) + 9); // +9 for ".symbols\0"
    sprintf(symbols_filename, "%s.symbols", bin_filename);

    FILE *symbols_file = fopen(symbols_filename, "w");
    if (symbols_file == NULL) {
        printf("Error: Could not create symbols file %s\n", symbols_filename);
        free(symbols_filename);
        return;
    }

    InstructionListNode_t *current = list;
    int addr = 0;
    while (current != NULL) {
        if (current->instruction->label != NULL) {
            fprintf(symbols_file, "%d %s\n", addr, current->instruction->label);
        }
        addr += 1;
        current = current->next;
    }

    fclose(symbols_file);
    printf("Symbols written to %s\n", symbols_filename);
    free(symbols_filename);
}

int main(int argc, char **argv) {
  if (argc != 3) {
    printf("usage: ./assembler [name].s [name].bin\n");
    exit(1);
  }

  FILE *src = fopen(argv[1], "r");
  if (src == NULL)
    exit(1);

  InstructionListNode_t *list = NULL;
  parse_instructions(src, &list);

  fclose(src);

  FILE *dst = fopen(argv[2], "w");
  if (dst == NULL)
    exit(1);

  InstructionListNode_t *current = list;
  while (current != NULL) {
    uint16_t compiled = compile_instruction(list, current->instruction);
    printf("instruction %10s: %d %10s      ->%#020b\n", current->instruction->label, current->instruction->type, current->instruction->arg, compiled);
    fwrite(&compiled, sizeof(uint16_t), 1, dst);
    current = current->next;
  }

  fclose(dst);
  printf("written to %s\n", argv[2]);
  write_symbols(list, argv[2]);
}
