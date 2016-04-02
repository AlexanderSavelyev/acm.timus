#include <iostream>
#include <utility>
#include <unordered_set>
#include <memory>
#include <vector>
#include <cmath>
#include <tuple>

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
   Circle(Circle&& other): center(other.center), radius(other.radius), vertices(std::move(other.vertices)) {
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
   virtual void processComponent(vector<int>& v) = 0;
};

class CircleBfsCallback : public BfsCallback {
public:
   vector<Circle>& circles;
   unordered_set<Point> comp_vertices;
   int res_count;
   CircleBfsCallback(vector<Circle>& circles): circles(circles),res_count(0){
   }
   virtual ~CircleBfsCallback(){}

   virtual void processComponent(vector<int>& cmp_vert) {
      comp_vertices.clear();
      int comp_edges = 0;
      for (auto idx : cmp_vert) {
         Circle& comp_cir = circles[idx];
         for (auto& comp_v : comp_cir.vertices) {
            comp_vertices.insert(comp_v);
         }

         comp_edges += comp_cir.getEdgesCount();
      }
      res_count += (comp_edges - comp_vertices.size() + 1);
   }

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
      while(left >= vSize() || right >= vSize()) {
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
typedef tuple<int, int, int> CircleIn;

namespace std {

   template <>
   struct hash<CircleIn> {
   public:
      size_t operator()(const CircleIn &p) const {
         size_t h1 = std::hash<int>()(std::get<0>(p));
         size_t h2 = std::hash<int>()(std::get<1>(p));
         size_t h3 = std::hash<int>()(std::get<2>(p));
         size_t ht = (h1 + h2) * h2 + h1;
         return (ht + h3) * h3 + ht;
      }
   };
}
class Task {
public:

   void run(istream& in, ostream& out) {
      int N, r, x, y;
      in >> N;
      
      vector<Circle> circles;

      Graph graph;
      unordered_set<CircleIn> cirq_unique;
      
      for (int i = 0; i < N; i++) {
         in >> x;
         in >> y;
         in >> r;
         CircleIn c_cur_in(x, y, r);
        
         if (cirq_unique.find(c_cur_in)!=cirq_unique.end()) {
            continue;
         } else {
            cirq_unique.insert(std::move(c_cur_in));
         }
         Circle c_cur(x, y, r);
         
         int cur_idx = circles.size();
         for (int j = 0; j < cur_idx; j++) {
            Circle& c_ex = circles[j];
            double d = c_cur.dist(c_ex);
            double r_sum = c_cur.radius + c_ex.radius;
            if (d <= r_sum) {
               double r_dif = fabs(c_cur.radius - c_ex.radius);
               if (d >= r_dif) {
                  c_cur.calculatePushVertex(c_ex);
                  graph.insertEdge(cur_idx, j);
               }
            }
         }
         circles.push_back(std::move(c_cur));
      }
      CircleBfsCallback cb(circles);
      graph.bfs(cb);
      int num_circles = cb.res_count + 1;
      out << num_circles;
      
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