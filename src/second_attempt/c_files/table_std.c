//__attribute__((import_module("host"), import_name("print"))) void print(char*);
//__attribute__((import_module("host"), import_name("print_num"))) void print_num(int);
//__attribute__((import_module("host"), import_name("exception"))) void exception(char*);
#include "table_std.h"

unsigned int strlen(const char *s)
{
    unsigned int count = 0;
    while(*s!='\0')
    {
        count++;
        s++;
    }
    return count;
}

char* strcpy(char* destination, const char* source)
{
    // return if no memory is allocated to the destination
    if (destination == NULL) {
        return NULL;
    }

    // take a pointer pointing to the beginning of the destination string
    char *ptr = destination;

    // copy the C-string pointed by source into the array
    // pointed by destination
    while (*source != '\0')
    {
        *destination = *source;
        destination++;
        source++;
    }

    // include the terminating null character
    *destination = '\0';

    // the destination is returned by standard `strcpy()`
    return ptr;
}

char* stralloc(const char* source) {
    char* buff = malloc(strlen(source)+1);
    strcpy(buff, source);
    return buff;
}

void print_value(Value* value) {
    switch(value->tag) {
        case NONE:
            print("Type: None, Value: None\n");
            break;
        case NUMBER:
            print("Type: Number, Value: ");
            print_num(value->variant.number);
            print("\n");
            break;
        case STRING:
            print("Type: String, Value: ");
            print(value->variant.string);
            print("\n");
            break;
        case CLOSURE:
            print("Type: Closure");
    }
}

Value test() {
    Value ret;
    ret.variant.string = stralloc("yo yo yo everybody");
    ret.tag = STRING;
    return ret;
}

Value* Number_new(int num) {
    Value* number = malloc(sizeof(Value));
    number->variant.number = num;
    number->tag = NUMBER;
    number->ref_count = 1;
    return number;
}
Value* String_new(const char* str) {
    Value* string = malloc(sizeof(Value));
    string->variant.string = stralloc(str);
    string->tag = STRING;
    string->ref_count = 1;
    return string;
}
Value* Closure_new(Closure closure) {
    Value* new_closure = malloc(sizeof(Value));
    new_closure->tag = CLOSURE;
    new_closure->ref_count = 1;
    new_closure->variant.closure = malloc(sizeof(Closure));
    new_closure->variant.closure->p = closure.p;
    new_closure->variant.closure->args = closure.args;
    return new_closure;
}
Value* None_new() {
    Value* none = malloc(sizeof(Value));
    none->tag = NONE;
    none->ref_count = 1;
    return none;
}

Value* None() {
    return None_new();
}

void decrement(Value* value) {
    value->ref_count -= 1;
    if(value->ref_count == 0) {
        print("should free this value here but not implemented yet\n");
    }
}
void increment(Value* value) {
    value->ref_count += 1;
}

Value* run_closure(Value* closure) {
    return (*closure->variant.closure->p)(closure->variant.closure->args);
}

Value* this_function(Value** args) {
    Value* arg1 = args[0];
    Value* arg2 = args[1];
    print("printing first arg: ");
    print_value(arg1);
    print("\nprinting second arg: ");
    print_value(arg2);
    print("\n");
    return None_new();
}
void run_test() {
    Closure my_closure;
    Value* my_num = Number_new(1);
    Value* second_num = Number_new(2);
    my_closure.args = malloc(sizeof(Value*));
    my_closure.p = &this_function;
    my_closure.args[0] = my_num;
    my_closure.args[1] = second_num;
    Value* some_closure = Closure_new(my_closure);
    print_value(run_closure(some_closure));
}

// NUMBER
    Value* Number_operation(Value* rhs, Value* lhs, enum NumberOperator number_operator) {
        switch(number_operator) {
            case ADD:
                return Number_new(rhs->variant.number+lhs->variant.number);
            break;
            case SUBTRACT:
                exception("subtract not implemented yet");
            break;
            case MULTIPLY:
                exception("multiply not implemented yet");
            break;
            case DIVIDE:
                exception("divide not implemented yet");
            break;
        }
    }