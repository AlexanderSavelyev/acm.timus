#include <iostream>

using namespace std;

class Task {
public:
   void run(istream& in, ostream& out) {
      int a, b;
      in >> a;
      in >> b;
      
      out << (a + b);
      out.flush();
   }
};

#ifdef ONLINE_JUDGE
int main() {
   Task task;
   task.run(cin, cout);
   
   return 0;
}
#endif