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
   private int nextRandom = 50000;

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


   public Pair<Integer, Integer> getRandomPoint() {
      nextRandom++;
      return new Pair(nextRandom, nextRandom);
   }

   public class Pair<A, B> {

      public A first;
      public B second;

      public Pair(A first, B second) {
         super();
         this.first = first;
         this.second = second;
      }

      public int hashCode() {
         int hashFirst = first != null ? first.hashCode() : 0;
         int hashSecond = second != null ? second.hashCode() : 0;
         return (hashFirst + hashSecond) * hashSecond + hashFirst;
      }

      public boolean equals(Object other) {
         Pair otherPair = (Pair) other;
         return ((this.first == otherPair.first
                 || (this.first != null && otherPair.first != null
                 && this.first.equals(otherPair.first)))
                 && (this.second == otherPair.second
                 || (this.second != null && otherPair.second != null
                 && this.second.equals(otherPair.second))));
      }

      public String toString() {
         return "(" + first + ", " + second + ")";
      }

   }

   public static float roundFloat(double in) {
      return ((int) ((in * 1000f) + 0.5f)) / 1000f;
   }
   public class Circle {

      public Pair<Integer, Integer> center;
      public int radius;
      public HashSet<Pair<Float, Float>> vertices = new HashSet<>();

      public Circle(int x, int y, int r) {
         center = new Pair(x, y);
         radius = r;
      }

      public void addVertex(Pair<Float, Float> b) {
         vertices.add(b);
      }
      public int getEdgesCount() {
         return vertices.size();
      }
      public boolean equals(Circle other) {
         return radius == other.radius && center.equals(other.center);
      }
      
      public LinkedList<Pair<Float,Float>> calculateVertex(Circle other) {
         LinkedList<Pair<Float,Float>> res = new LinkedList<>();
         double d = dist(other);
         
         int dx = other.center.first - center.first;
         int dy = other.center.second - center.second;
         double base = Math.PI / 2.0f;
         if(dx != 0) {
            base = Math.signum(dy + 0.1)*Math.acos((dx*dx + d*d - dy*dy)/(2*dx*d));
         }
         //System.out.println("base = " + base);
         double diff = Math.acos((radius * radius + d * d - other.radius * other.radius)/(2*radius*d));
         double a1 = base + diff;
         double a2 = base - diff;
         
         res.add(calcVert(a1));
         if(a2 != a1) {
            res.add(calcVert(a2));
         }
         return res;
      }
      
      public double dist(Circle c2) {
         Circle c1 = this;
         return Math.sqrt(Math.pow(c1.center.first - c2.center.first, 2) + Math.pow(c1.center.second - c2.center.second, 2));
      }

      private Pair<Float, Float> calcVert(double angle) {
         return new Pair(roundFloat(center.first + radius * Math.cos(angle)), roundFloat(center.second + radius * Math.sin(angle)));
      }
   }

   class Graph {
      class Node {

         public BitSet nei;

         public Node(int N) {
            nei = new BitSet(N);
         }
      }

      public ArrayList<Node> nodes;
      private final int N;
      private final boolean[] marked;  // marked[v] = is there an s-v path
      LinkedList<Integer> q = new LinkedList<>();

      public Graph(int N) {
         this.N = N;
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
         int  c_res=0;
         for (int v = 0; v < circles.size(); v++) {
            if (!marked[v]) {
               HashSet<Integer> comp = bfs(v);
               HashSet<Pair<Float, Float>> comp_vertices = new HashSet<>();
               int comp_edges = 0;
               for(Integer c : comp) {
                  Circle comp_cir = circles.get(c);
                  for(Pair<Float, Float> comp_v: comp_cir.vertices) {
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
      for (int i = 0; i < N; i++) {
         Circle c_cur = new Circle(nextInt(), nextInt(), nextInt());
         Pair p_cur = new Pair(c_cur.radius, c_cur.center);
         if(cirq_unique.contains(p_cur)) {
            continue;
         } else {
            cirq_unique.add(p_cur);
         }
         circles.add(c_cur);
         int cur_idx = circles.size() - 1;
         for (int j = 0; j < cur_idx; j++) {
            Circle c_ex = circles.get(j);
            double d = c_cur.dist(c_ex);
            int r_sum = c_cur.radius + c_ex.radius;
            if (d <= r_sum) {
               int r_dif = Math.abs(c_cur.radius - c_ex.radius);
               if (d >= r_dif) {
                  LinkedList<Pair<Float, Float>> ve = c_cur.calculateVertex(c_ex);
                  
//                  LinkedList<Pair<Float, Float>> ve2 = c_ex.calculateVertex(c_cur);
//                  HashSet<Pair<Float, Float>> checkList = new HashSet<>();
//                  checkList.addAll(ve2);
                  
                  for(Pair<Float, Float> vp : ve) {
                     c_cur.addVertex(vp);
                     c_ex.addVertex(vp);
//                     if(!checkList.contains(vp)) {
//                        throw new RuntimeException(ve.toString() + "\nNOT EQ\n" + ve2.toString());
//                     }
                  }
                  
                  
                  
                  graph.insertEdge(cur_idx, j);
               }

            }
         }
      }
      out.println(graph.bfs(circles));
   }

}
