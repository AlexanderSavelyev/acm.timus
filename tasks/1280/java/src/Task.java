
import java.io.*;
import java.util.ArrayList;
import java.util.BitSet;
import java.util.LinkedList;

/**
 *
 */
public class Task {

   /**
    * @param args the command line arguments
    */
   public static void main(String[] args) throws IOException {
      new Task().run();
   }
   StreamTokenizer in;
   PrintWriter out;

   int nextInt() throws IOException {
      in.nextToken();
      return (int) in.nval;
   }

   void run() throws IOException {
      boolean oj = System.getProperty("ONLINE_JUDGE") != null;
      Reader reader = oj ? new InputStreamReader(System.in) : new FileReader("input1.txt");
      //Writer writer = oj ? new OutputStreamWriter(System.out) : new FileWriter("output.txt");\
      Writer writer = new OutputStreamWriter(System.out);
      in = new StreamTokenizer(new BufferedReader(reader));
      out = new PrintWriter(writer);

      solve();
      out.flush();
   }

   class Node {

      public BitSet nei;

      public Node(int N) {
         nei = new BitSet(N);
      }

   }

   class Graph {

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
         marked= new boolean[N];
      }

      public boolean insertEdge(int left, int right) {
         if (nodes.get(right).nei.get(left)) {
            return false;
         }
//         if (pathExists(right, left)) {
//            return false;
//         }
         nodes.get(left).nei.set(right);
         return true;
      }
      
      public boolean cycleExists() {
         for (int v = 0; v < N; v++) {
            if(pathExists(v, v)) {
               return true;
            }
         }
         return false;
      }

      private boolean pathExists(int from, int to) {

         q.clear();
         for (int v = 0; v < N; v++) {
            marked[v] = false;
         }
         marked[from] = true;
         q.add(from);

         while (!q.isEmpty()) {
            int v = q.pollLast();
            BitSet bs = nodes.get(v).nei;
            for (int w = bs.nextSetBit(0); w >= 0; w = bs.nextSetBit(w + 1)) {
               if (w == to) {
                  return true;
               }
               if (!marked[w]) {
                  marked[w] = true;
                  q.add(w);
               }
            }
         }
//         BitSet bs = nodes.get(from).nei;
//         for (int i = bs.nextSetBit(0); i >= 0; i = bs.nextSetBit(i + 1)) {
//            if (i == to) {
//               return true;
//            }
//            if (pathExists(i, to)) {
//               return true;
//            }
//         }
         return false;
      }

      private boolean edgeExists(int a, int b) {
         return nodes.get(a).nei.get(b);
      }

   }

   void solve() throws IOException {
      int N = nextInt();
      int M = nextInt();
      Graph graph = new Graph(N);

      for (int i = 0; i < M; i++) {
         int s = nextInt() - 1;
         int u = nextInt() - 1;
         if (!graph.insertEdge(s, u)) {
            out.println("NO");
            return;
         }
      }
//      if(graph.cycleExists()) {
//         out.println("NO");
//         return;
//      }
      int[] seq = new int[N];
      for (int i = 0; i < N; i++) {
         seq[i] = nextInt() - 1;
      }
      for (int i = 0; i < N - 1; i++) {
         //int j = i + 1;
         for (int j = i + 1; j < N; j++) {
//            if (!graph.insertEdge(seq[i], seq[j])) {
//               out.println("NO");
//               return;
//            }
            if(graph.edgeExists(seq[j], seq[i])) {
               out.println("NO");
               return;
            }
         }
      }
//      if(graph.cycleExists()) {
//         out.println("NO");
//         return;
//      }
      out.println("YES");
   }

}
