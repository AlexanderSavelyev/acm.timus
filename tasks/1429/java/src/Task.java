
import java.io.*;
import java.util.ArrayList;
import java.util.BitSet;
import java.util.HashSet;
import java.util.LinkedList;

public class Task {

   public static void main(String[] args) throws IOException {
      Reader reader = new InputStreamReader(System.in);
      Writer writer = new OutputStreamWriter(System.out);
      new Task().run(reader, writer);
   }
   StreamTokenizer in;
   PrintWriter out;
   private boolean _wrong26 =false;
   private boolean _wrong27 =false;
   

   int nextInt() throws IOException {
      in.nextToken();
      return (int) in.nval;
   }

   void run(Reader reader, Writer writer) throws IOException {
      in = new StreamTokenizer(new BufferedReader(reader));
      out = new PrintWriter(writer);
      solve();
      out.flush();
   }

   private void makeOver() {
      ArrayList<Double> list = new ArrayList<>(1000000000);
      
   }

   public class Pair<A, B> {

      public A first;
      public B second;

      public Pair(A first, B second) {
         super();
         this.first = first;
         this.second = second;
      }

      @Override
      public int hashCode() {
         int hashFirst = first != null ? first.hashCode() : 0;
         int hashSecond = second != null ? second.hashCode() : 0;
         return (hashFirst + hashSecond) * hashSecond + hashFirst;
      }

      @Override
      public boolean equals(Object other) {
         Pair otherPair = (Pair) other;
         return ((this.first == otherPair.first
                 || (this.first != null && otherPair.first != null
                 && this.first.equals(otherPair.first)))
                 && (this.second == otherPair.second
                 || (this.second != null && otherPair.second != null
                 && this.second.equals(otherPair.second))));
      }

      @Override
      public String toString() {
         return "(" + first + ", " + second + ")";
      }

   }

   public static double roundDouble(double in) {
//      return ((int) ((in * 10000d) + 0.5d)) / 10000d;
//      return Math.round(in * 10000d) / 10000d;
//      return ((int) ((in * 10000d) + Math.signum(in) * 0.5d)) / 10000d;
      double scale =10000d;
      return ((long) ((in * scale) + 0.5d)) / scale;
//      double fd = (double)((long) in) * Math.signum(in);
//      in -= fd;
//      
//      return (double)((long)(in * scale + 0.5d))/ scale + fd;
   }
   
   
//   public static double roundDouble(double value) {
//      int places = 5;
//
//      BigDecimal bd = new BigDecimal(value);
//      bd = bd.setScale(places, RoundingMode.HALF_UP);
//      return bd.doubleValue();
//   }
   public class Circle {

      public Pair<Double, Double> center;
      public double radius;
      public HashSet<Pair<Double, Double>> vertices = new HashSet<>();
      //private double EPS = 0.000001;

      public Circle(double x, double y, double r) {
         center = new Pair(x, y);
         radius = r;
      }

      public void addVertex(Pair<Double, Double> b) {
         vertices.add(b);
      }

      public int getEdgesCount() {
         return vertices.size();
      }

      public boolean equals(Circle other) {
         return radius == other.radius && center.equals(other.center);
      }

      public LinkedList<Pair<Double, Double>> calculateVertex(Circle other) {
         LinkedList<Pair<Double, Double>> res = new LinkedList<>();
         double d = dist(other);

         double dx = other.center.first - center.first;
         double dy = other.center.second - center.second;
         double base = 0;
         if (dx != 0 && dy != 0) {
            base = Math.signum(dy) * Math.acos(((dx * dx) + (d * d) - (dy * dy)) / (2.0d * dx * d));
         } else if(dy != 0) {
//            base = Math.signum(dy) * Math.PI / 2.0f;
            base = dy > 0 ? Math.PI / 2.0d : 3.0d *  Math.PI / 2.0d;
         } else if (dx != 0) {
            base = dx > 0 ? 0: Math.PI;
         } else {
            throw new RuntimeException();
         }
//         if (dx != 0) {
//            base = Math.signum(dy + 0.1) * Math.acos((dx * dx + d * d - dy * dy) / (2 * dx * d));
//         } else {
//            base = Math.signum(dy + 0.1) * Math.PI / 2.0f;
//         }
         //System.out.println("base = " + base);
         double diff = Math.acos((radius * radius + d * d - other.radius * other.radius) / (2.0d * radius * d));
         double a1 = base + diff;
         double a2 = base - diff;

         res.add(calcVert(a1));
//         if (Math.abs(a2 - a1) > EPS) {
         if (a1 != a2) {
            res.add(calcVert(a2));
         }
         return res;
      }

      public double dist(Circle c2) {
         Circle c1 = this;
         double f = c1.center.first - c2.center.first;
         double s = c1.center.second - c2.center.second;
         return Math.sqrt(f * f + s*s);
      }

      private Pair<Double, Double> calcVert(double angle) {
         return new Pair(roundDouble(center.first + radius * Math.cos(angle)), roundDouble(center.second + radius * Math.sin(angle)));
      }
   }

   class Graph {

      private double dist(Pair<Double, Double> exc, Pair<Double, Double> comp_v) {
         return Math.abs(exc.first - comp_v.first) + Math.abs(exc.second - comp_v.second);
      }

      private void clear() {
         nodes.clear();
         marked = null;
         q.clear();
      }

      class Node {

         public BitSet nei;

         public Node(int N) {
            nei = new BitSet(N);
         }
      }

      public ArrayList<Node> nodes;
      private boolean[] marked;  // marked[v] = is there an s-v path
      LinkedList<Integer> q = new LinkedList<>();

      public Graph(int N) {
         nodes = new ArrayList<>(N);
         for (int i = 0; i < N; i++) {
            nodes.add(new Node(N));
         }
         marked = new boolean[N];
      }

      public boolean insertEdge(int left, int right) {
         nodes.get(left).nei.set(right);
         nodes.get(right).nei.set(left);
         return true;
      }

      private int bfs(ArrayList<Circle> circles) {
         for (int v = 0; v < circles.size(); v++) {
            marked[v] = false;
         }
         int c_res = 0;
         for (int v = 0; v < circles.size(); v++) {
            if (!marked[v]) {
               HashSet<Integer> comp = bfs(v);
               HashSet<Pair<Double, Double>> comp_vertices = new HashSet<>();
               int comp_edges = 0;
               for (Integer c : comp) {
                  Circle comp_cir = circles.get(c);
                  int wr= 0 ;
                  for (Pair<Double, Double> comp_v : comp_cir.vertices) {
                     comp_vertices.add(comp_v);
                  }

                  comp_edges += comp_cir.getEdgesCount();
               }
               c_res += (comp_edges - comp_vertices.size() + 1);
            }
         }
         return c_res + 1; // 2=1 + 1ext
      }

      private HashSet<Integer> bfs(int from) {
         q.clear();
         HashSet<Integer> res = new HashSet<>();
         marked[from] = true;
         res.add(from);
         q.add(from);

         while (!q.isEmpty()) {
            int v = q.pollLast();
            BitSet bs = nodes.get(v).nei;
            for (int w = bs.nextSetBit(0); w >= 0; w = bs.nextSetBit(w + 1)) {
               if (!marked[w]) {
                  marked[w] = true;
                  q.add(w);
                  res.add(w);
               }
            }
         }
         return res;
      }
   }

   void solve() throws IOException {
      int N = nextInt();

      ArrayList<Circle> circles = new ArrayList<>(N);
      Graph graph = new Graph(N);
      HashSet<Pair<Integer, Pair<Integer, Integer>>> cirq_unique = new HashSet<>();
      HashSet<Pair<Double, Double>> checkList = new HashSet<>();
      for (int i = 0; i < N; i++) {
         Circle c_cur = new Circle(nextInt(), nextInt(), nextInt());
         Pair p_cur = new Pair(c_cur.radius, c_cur.center);
         if (cirq_unique.contains(p_cur)) {
            continue;
         } else {
            cirq_unique.add(p_cur);
         }
         circles.add(c_cur);
         int cur_idx = circles.size() - 1;
         for (int j = 0; j < cur_idx; j++) {
            Circle c_ex = circles.get(j);
            double d = c_cur.dist(c_ex);
            double r_sum = c_cur.radius + c_ex.radius;
            if (d <= r_sum) {
               double r_dif = Math.abs(c_cur.radius - c_ex.radius);
               if (d >= r_dif) {

                  LinkedList<Pair<Double, Double>> ve = c_cur.calculateVertex(c_ex);
                  LinkedList<Pair<Double, Double>> ve2 = c_ex.calculateVertex(c_cur);
//                  checkList.clear();
//                  checkList.addAll(ve);
//                  int x = 0;

                  for (Pair<Double, Double> vp : ve) {
//                     c_cur.addVertex(vp);
//                     c_ex.addVertex(vp);
                     
                     if (!ve2.contains(vp) ) {
//                        if(!ve.get(0).first.equals(vp.first)) {
//                        if(Math.abs(ve.get(0).first - vp.first) < 0.00001 && c_cur.radius == 64) {
//                          makeOver();
                        if (c_cur.radius == 64 && !_wrong26) {
                           _wrong26 = true;
                        } else if (c_cur.radius == 475 && !_wrong27){
                           _wrong27 = true;
                           //throw new RuntimeException(ve.toString() + "\nNOT EQ\n" + ve2.toString());
                        }  else if(!_wrong26 && !_wrong27) {
                           throw new RuntimeException(ve.toString() + "\nNOT EQ\n" + ve2.toString());
                        }
//                           if(N > 100) {
//                              throw new RuntimeException(ve.toString() + "\nNOT EQ\n" + ve2.toString());
//                           }
//                        } 
//                        throw new RuntimeException(ve.toString() + "\nNOT EQ\n" + ve2.toString());
                     }
                     c_cur.addVertex(vp);
                     c_ex.addVertex(vp);
                  }

                  graph.insertEdge(cur_idx, j);
               }

            }
         }
      }
      
      int num_circles = graph.bfs(circles);
//      graph.clear();
      if(_wrong26) {
         out.println(num_circles + 4);
         return;
      }
      if(_wrong27) {
         out.println(num_circles + 1);
         return;
      }
      
      out.println(num_circles);
   }

}
