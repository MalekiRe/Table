#pragma once
__attribute__((import_module("host"), import_name("print"))) void print(char*);
__attribute__((import_module("host"), import_name("print_num"))) void print_num(int);
__attribute__((import_module("host"), import_name("exception"))) void exception(char*);
#include "shared_std.h"
#include "walloc.h"

unsigned int strlen(const char *s);

char* strcpy(char* destination, const char* source);

void print(char*);
void print_num(int);
void exception(char*);

typedef enum TypeTag {
    NONE,
    STRING,
    NUMBER,
} TypeTag;

typedef union TypeVariant {
    char* string;
    int number;
} TypeVariant;

typedef struct Value {
    TypeVariant variant;
    TypeTag tag;
    int ref_count;
} Value;


Value Number_new(int num);
Value String_new(const char* str);

void ref_dec(Value *value);
void ref_inc(Value *value);

void print_value(Value value);

Value test();