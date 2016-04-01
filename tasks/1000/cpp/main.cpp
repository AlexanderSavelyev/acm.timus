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

struct NonCopyable {
    NonCopyable() = default;
    NonCopyable(const NonCopyable&) = delete;
    NonCopyable & operator=(const NonCopyable&) = delete;
};

class Circle: public NonCopyable  {
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
class BfsCallback {
public:
   virtual void processComponent(vector<int>& v);
   virtual ~BfsCallback();
};


class Graph: public NonCopyable {
public:
   class Node: public NonCopyable  {
   public:
      unordered_set<int> neighbors;
      Node() {
      }
      Node(Node&& other): neighbors(std::move(other.neighbors)) {
      }
   };
//   class Edge: public NonCopyable  {
//   public:
//      int left;
//      int right;
//      Edge() {
//      }
//   };
   //vector<Edge> edges;
   vector<Node> nodes;
   Graph() {
   }

   Graph(int N) {
      for (int i = 0; i < N; i++) {
         addNode();
      }
   }

   int addNode() {
      Node node;
      nodes.push_back(std::move(node));
      return nodes.size() - 1;
   }
   int vSize() const {return nodes.size();}

   bool insertEdge(int left, int right) {
      int N = vSize();
      while(left >= N || right >= N) {
         addNode();
      }
      auto& node = nodes[left];
      auto f = node.neighbors.find(right);
      if(f == node.neighbors.end()) {
         node.neighbors.insert(right);
         auto& node2 = nodes[right];
         node2.neighbors.insert(left);
      } else {
         return false;
      }
      return true;
   }
   
   void bfs(BfsCallback& cb) {
      int N = vSize();
      vector<bool> marked(N);
//      marked.reset();
      
      vector<int> comp;
      
      for(int from = 0; from < N; from++) {
         if(!marked[from]) {
            comp.clear();
            bfsComp(from, marked, comp);
            cb.processComponent(comp);
         }
      }
   }

   void bfsComp(int from, vector<bool>& marked, vector<int>& comp) {
      std::vector<int> q;
      marked[from] =true;
      comp.push_back(from);
      q.push_back(from);

      while (!q.empty()) {
         int v = q.back();
         q.pop_back();
         auto& nei = nodes[v].neighbors;
         for (const auto& w : nei) {
            if (!marked[w]) {
               marked[w] =true;
               q.push_back(w);
               comp.push_back(w);
            }
         }
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