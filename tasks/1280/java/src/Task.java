
import java.io.*;
import java.util.ArrayList;
import java.util.BitSet;

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
      Reader reader = oj ? new InputStreamReader(System.in) : new FileReader("input2.txt");
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

      public Graph(int N) {
         nodes = new ArrayList<>(N);
         for (int i = 0; i < N; i++) {
            nodes.add(new Node(N));
         }
      }

      public boolean insertEdge(int left, int right) {
         if (nodes.get(right).nei.get(left)) {
            return false;
         }
         if (pathExists(right, left)) {
            return false;
         }
         nodes.get(left).nei.set(right);
         return true;
      }

      private boolean pathExists(int from, int to) {
         BitSet bs = nodes.get(from).nei;
         for (int i = bs.nextSetBit(0); i >= 0; i = bs.nextSetBit(i + 1)) {
            if (i == to) {
               return true;
            }
            if (pathExists(i, to)) {
               return true;
            }
         }
         return false;
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
      int[] seq = new int[N];
      for (int i = 0; i < N; i++) {
         seq[i] = nextInt() - 1;
      }
      for (int i = 0; i < N - 1; i++) {
         for (int j = i + 1; j < N; j++) {
            if (!graph.insertEdge(seq[i], seq[j])) {
               out.println("NO");
               return;
            }
         }
      }
      out.println("YES");
   }

}
