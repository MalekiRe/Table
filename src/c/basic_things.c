struct BigDecimal {
    long first;
    long second;
};

union TableTypeUnion {
    struct BigDecimal number;
    char* string;

};

enum TableTypeTag {
    NUMBER = 0,
    STRING = 1,

};