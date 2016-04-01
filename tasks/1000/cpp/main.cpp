#include <iostream>
#include <utility>
#include <unordered_set>
#include <memory>
#include <vector>
#include <cmath>

using namespace std;

typedef pair<double, double> Point;



namespace std {

   template <>
   struct hash<Point> {
   public:
      size_t operator()(const Point &p) const {
         size_t h1 = std::hash<double>()(p.first);
         size_t h2 = std::hash<double>()(p.second);
         return (h1 + h2) * h2 + h1;
      }
   };
}

class Circle {
public:
   Point center;
   double radius;
   unordered_set<Point> vertices;

   Circle(double x, double y, double r) : center(x, y), radius(r) {
   }

   void addVertex(double x, double y) {
      vertices.insert(make_pair(x, y));
   }

   size_t getEdgesCount() {
      return vertices.size();
   }

    double dist(Circle& c2) {
      double f = center.first - c2.center.first;
      double s = center.second - c2.center.second;
      return sqrt(f * f + s * s);
   }

   double roundDouble(double in) {
      double scale = 10000.0;
      return ((long long) ((in * scale) + 0.5)) / scale;
   }

   Point calcPoint(double angle) {
      return Point(roundDouble(center.first + radius * cos(angle)),
              roundDouble(center.second + radius * sin(angle)));
   }
   void calculatePushVertex(Circle& other) {
      double d = dist(other);

      double dx = other.center.first - center.first;
      double dy = other.center.second - center.second;
      double base = 0;
      if (dx != 0 && dy != 0) {
         base = copysign(1.0, dy) * acos(((dx * dx) + (d * d) - (dy * dy)) / (2.0d * dx * d));
      } else if (dy != 0) {
         base = dy > 0 ? M_PI / 2.0d : 3.0d * M_PI / 2.0d;
      } else if (dx != 0) {
         base = dx > 0 ? 0 : M_PI;
      } else {
         //throw new RuntimeException();
      }
      double diff = acos((radius * radius + d * d - other.radius * other.radius) / (2.0d * radius * d));
      double a1 = base + diff;
      double a2 = base - diff;

      
      auto p1 = calcPoint(a1);
      vertices.insert(p1);
      other.vertices.insert(p1);
      if (a1 != a2) {
         auto p2 = calcPoint(a2);
         vertices.insert(p2);
         other.vertices.insert(p2);
      }
   }
};

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