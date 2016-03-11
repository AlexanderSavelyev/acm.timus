
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

   private double dist(Circle c1, Circle c2) {
      return Math.sqrt(Math.pow(c1.center.first - c2.center.first, 2) + Math.pow(c1.center.second - c2.center.second, 2));
   }

   private Pair<Integer, Integer> getRandomPoint() {
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

   class Circle {

      public Pair<Integer, Integer> center;
      public int radius;
      public HashSet<Pair<Integer, Integer>> borders = new HashSet<>();
      public HashSet<Pair<Integer, Integer>> vertices = new HashSet<>();

      private Circle(int x, int y, int r) {
         center = new Pair(x, y);
         radius = r;
         borders.add(new Pair(x + r, y));
         borders.add(new Pair(x, y - r));
         borders.add(new Pair(x - r, y));
         borders.add(new Pair(x, y + r));
      }

      private void addVertex(Pair<Integer, Integer> b) {
         vertices.add(b);
      }
      public int getEdgesCount() {
         return vertices.size();
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
         for (int v = 0; v < N; v++) {
            marked[v] = false;
         }
         int  c_res=0;
         for (int v = 0; v < N; v++) {
            if (!marked[v]) {
               HashSet<Integer> comp = bfs(v);;
               HashSet<Pair<Integer, Integer>> comp_vertices = new HashSet<>();
               int comp_edges = 0;
               for(Integer c : comp) {
                  Circle comp_cir = circles.get(c);
                  for(Pair<Integer, Integer> comp_v: comp_cir.vertices) {
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
//            if (w == to) {
//               return true;
//            }
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
      for (int i = 0; i < N; i++) {
         Circle c_cur = new Circle(nextInt(), nextInt(), nextInt());
         circles.add(c_cur);
         for (int j = 0; j < i; j++) {
            Circle c_ex = circles.get(j);
            double d = dist(c_cur, c_ex);
            int r_sum = c_cur.radius + c_ex.radius;
            if (d <= r_sum) {
               int num_b = 0;
               for (Pair<Integer, Integer> b : c_cur.borders) {
                  if (c_ex.borders.contains(b)) {
                     num_b++;
                     c_ex.addVertex(b);
                     c_cur.addVertex(b);
                  }
               }
               int r_dif = Math.abs(c_cur.radius - c_ex.radius);
               if (d == r_sum) {
                  if (num_b == 0) {
                     throw new RuntimeException("Incorrect algorithm");
                  }
               } else if (d == r_dif) {
                  if (num_b == 0) {
                     throw new RuntimeException("Incorrect algorithm");
                  }
               } else if (d > r_dif) {
                  for (int p = num_b; p < 2; p++) {
                     Pair<Integer, Integer> rp = getRandomPoint();
                     c_ex.addVertex(rp);
                     c_cur.addVertex(rp);
                  }
               }
               if (d >= r_dif) {
                  graph.insertEdge(i, j);
               }

            }
         }
      }
      out.println(graph.bfs(circles));
   }

}
