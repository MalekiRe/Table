__attribute__((import_module("host"), import_name("print"))) void print(char*);
__attribute__((import_module("host"), import_name("print_num"))) void print_num(long, long);
__attribute__((import_module("host"), import_name("exception"))) void exception(char*);

#include "walloc.c"

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


struct BigDecimal {
    long first;
    long second;
};

union TableTypeUnion {
    char none;
    struct BigDecimal number;
    char* string;
};

enum TableTypeTag {
    NONE = 0,
    NUMBER = 1,
    STRING = 2,
};

typedef struct TableType {
    enum TableTypeTag table_type_tag;
    union TableTypeUnion table_type_union;
    int ref_count;
} TableType;


void print_table_type(struct TableType table_type) {
    switch(table_type.table_type_tag) {
        case NONE: print("none"); break;
        case NUMBER: print("number: "); print_num(table_type.table_type_union.number.first, table_type.table_type_union.number.second); break;
        case STRING: print(table_type.table_type_union.string); break;
    }
}

struct TableType create_big_number(long first, long second) {
    struct TableType big_number;
    big_number.table_type_tag = NUMBER;
    struct BigDecimal num;
    num.first = first;
    num.second = second;
    big_number.table_type_union.number = num;
    return big_number;
};

void inc_ref_count(struct TableType *table_type);
void dec_ref_count(struct TableType *table_type);
void free_table_type(struct TableType *table_type);

struct TableType create_string(char *string) {
    struct TableType string_type;
    string_type.table_type_tag = STRING;
    string_type.table_type_union.string = malloc(strlen(string) + 1);
    strcpy(string_type.table_type_union.string, string);
    inc_ref_count(&string_type);
    return string_type;
}

void inc_ref_count(struct TableType *table_type) {
    table_type->ref_count++;
}
void dec_ref_count(struct TableType *table_type) {
    table_type->ref_count--;
    if(table_type->ref_count == 0) {
       free_table_type(table_type);
    }
}
void free_table_type(struct TableType *table_type) {
    print("supposed to free here, rn does nothing");
}

enum TableOperators {
    ADD,
    SUBTRACT,
    MULTIPLY,
    DIVIDE,
};

struct TableType table_operator(struct TableType lhs, struct TableType rhs, enum TableOperators table_operator) {
    if(lhs.table_type_tag != NUMBER) {
        exception("error not a number in operator exp");
    }
    if(rhs.table_type_tag != NUMBER) {
        exception("error not a number in operator exp");
    }
    switch (table_operator) {
        case ADD:
        lhs.table_type_union.number.first += rhs.table_type_union.number.first;
        lhs.table_type_union.number.second += rhs.table_type_union.number.second;
        break;
        case SUBTRACT:
        lhs.table_type_union.number.first -= rhs.table_type_union.number.first;
        lhs.table_type_union.number.second -= rhs.table_type_union.number.second;
        break;
        case MULTIPLY:
        lhs.table_type_union.number.first *= rhs.table_type_union.number.first;
        lhs.table_type_union.number.second *= rhs.table_type_union.number.second;
        break;
        case DIVIDE:
        lhs.table_type_union.number.first /= rhs.table_type_union.number.first;
        lhs.table_type_union.number.second /= rhs.table_type_union.number.second;
        break;
    }
    return lhs;
}

int add(int a, int b) {
    return a + b;
}

