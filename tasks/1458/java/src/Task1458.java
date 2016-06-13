
import java.io.*;
import java.util.BitSet;
import java.util.LinkedList;

public class Task1458 {

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

   public static void main(String[] args) throws IOException {
//      Reader reader = new InputStreamReader(System.in);
//Writer writer = new OutputStreamWriter(System.out);
      Reader reader = new FileReader("../visual/test1.txt");
      StringWriter writer = new StringWriter();
      
      new Task1458().run(reader, writer);
      
   }
   BufferedReader in;
   PrintWriter out;

   void run(Reader reader, Writer writer) throws IOException {
      in = new BufferedReader(reader);
      out = new PrintWriter(writer);
      solve();
      out.flush();
   }

   void solve() throws IOException {
      int N = Integer.parseInt(in.readLine());

      BitSet solutionW = new BitSet(N * N);
      BitSet solutionB = new BitSet(N * N);
      
      

      for (int i = 0; i < N; i++) {
         String tokens = in.readLine();
         boolean flipW = false;
         boolean flipB = false;
         for (int j = 0; j < tokens.length(); ++j) {
            if (tokens.charAt(j) == 'W') {
               for (int si = 0; si < N; si++) {
                  if (si == i) {
                     continue;
                  }
                  addMergeSolution(solutionW, si, j, N);
               }
//               addFlipSolution(solutionW, i, N);
//               for (int sj = 0; sj < N; sj++) {
//                  addMergeSolution(solutionW, i, sj, N);
//               }
               flipW = !flipW;
            } else {
               for (int si = 0; si < N; si++) {
                  if (si == i) {
                     continue;
                  }
                  addMergeSolution(solutionB, si, j, N);
               }
//                  addFlipSolution(solutionB, i, N);
//               for (int sj = 0; sj < N; sj++) {
//                  addMergeSolution(solutionB, i, sj, N);
//               }
               flipB = !flipB;
            }
         }
         
         if(flipW) {
            for (int sj = 0; sj < N; sj++) {
                  addMergeSolution(solutionW, i, sj, N);
               }
         }
         
         if(flipB) {
            for (int sj = 0; sj < N; sj++) {
                  addMergeSolution(solutionB, i, sj, N);
               }
         }
      }

      LinkedList<Integer> res1 = getSize(solutionB);
      LinkedList<Integer> res2 = getSize(solutionW);
      LinkedList<Integer> res = res1;
      if (res2.size() < res1.size()) {
         res = res2;
      }
      out.println(res.size());
      for (Integer x : res) {
         out.println((x / N + 1) + " " + (x % N + 1));
//         out.println((pair.first + 1) + " " + (pair.second + 1));
      }

   }

   private void addMergeSolution(BitSet solution, int i, int j, int N) {
      solution.flip(i * N + j);
   }

   private void addFlipSolution(BitSet solution, int i, int N) {
      solution.flip(i * N + 1, i * N + N);
   }

   private LinkedList<Integer> getSize(BitSet sol) {
      LinkedList<Integer> res = new LinkedList<>();
      for (int x = sol.nextSetBit(0); x >= 0; x = sol.nextSetBit(x + 1)) {
         res.add(x);
      }
      return res;
   }

}
