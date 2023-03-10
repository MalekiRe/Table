#pragma once
__attribute__((import_module("host"), import_name("print"))) void print(char*);
__attribute__((import_module("host"), import_name("print_num"))) void print_num(int);
__attribute__((import_module("host"), import_name("print_boolean"))) void print_boolean(int);
__attribute__((import_module("host"), import_name("exception"))) void exception(char*);
#include "shared_std.h"
#include "walloc.h"

unsigned int strlen(const char *s);

char* strcpy(char* destination, const char* source);

void print(char*);
void print_num(int);
void print_boolean(bool boolean);
void exception(char*);

typedef struct VALUE Value;

typedef struct CLOSURE {
    Value* (*p)();
    Value** args;
} Closure;


typedef enum TYPE_TAG {
    NONE,
    STRING,
    NUMBER,
    CLOSURE,
    BOOLEAN,
} TypeTag;

typedef union TYPE_VARIANT {
    char* string;
    int number;
    Closure* closure;
    bool boolean;
} TypeVariant;

typedef struct VALUE {
    TypeVariant variant;
    TypeTag tag;
    int ref_count;
} Value;


Value* Number_new(int num);
Value* String_new(const char* str);
Value* Closure_new(Closure closure);
Value* Boolean_new(bool boolean);
Value* None_new();

Value* None();

void decrement(Value* value);
void increment(Value* value);

void print_value(Value* value);

Value* run_closure(Value* closure);

Value test();

void run_test();

// Number
    enum NumberOperator {
        ADD,
        SUBTRACT,
        DIVIDE,
        MULTIPLY
    };
    Value* Number_operation(Value* rhs, Value* lhs, enum NumberOperator number_operator);