#include <iostream>
#include <sstream>
#include <string>
#include <assert.h>
#include <cstring>
#include "main.cpp"

bool testAssert(const char* exp, stringstream& res) {
   return res.str().compare(exp) == 0;
}

string toString(stringstream& res) {
   return res.str();
}
#define assertEq(expr1, expr2) \
    do { \
        if (false == testAssert(expr1, expr2)) { \
            std::cerr <<"ASSERTION FAILED: " \
                      << ::basename(__FILE__) << ":" << __LINE__ \
                      << ", expected '" << expr1 << "' but was '" << toString(expr2)<< "'" << std::endl; \
        } \
    } while (0)

using namespace std;

int main() {
   Task task;
   stringstream s_in;
   s_in << "3\n"
           "-1 0 2\n"
           "1 0 2\n"
           "0 0 3";
   
   stringstream s_out;
   task.run(s_in, s_out);
   
   assertEq("6", s_out);
   
   return 0;
}