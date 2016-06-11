
import java.io.*;
import java.util.ArrayList;
import java.util.BitSet;
import java.util.HashSet;

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
      Reader reader = new InputStreamReader(System.in);
      Writer writer = new OutputStreamWriter(System.out);
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
      int num_columns = Integer.parseInt(in.readLine());

      HashSet<Pair<Integer, Integer>> solutionW = new HashSet<>();
      HashSet<Pair<Integer, Integer>> solutionB= new HashSet<>();
      
      for (int i = 0; i < num_columns; i++) {
         String tokens = in.readLine();
         for (int j = 0; j < tokens.length(); ++j) {
            if (tokens.charAt(j) == 'W') {
               for(int si = 0; si < num_columns; si++) {
                  addMergeSolution(solutionW, si, j);
               }
               for(int sj = 0; sj < num_columns; sj++) {
                  if(sj == j)
                     continue;
                  addMergeSolution(solutionW, i, sj);
               }
            } else {
                  for(int si = 0; si < num_columns; si++) {
                  addMergeSolution(solutionW, si, j);
               }
               for(int sj = 0; sj < num_columns; sj++) {
                  if(sj == j)
                     continue;
                  addMergeSolution(solutionW, i, sj);
               }
            }
         }
      }
      
      out.println(solutionW.size());
      
      for (Pair<Integer, Integer> pair : solutionW) {
         out.println((pair.first + 1) + " " + (pair.second + 1));
      }

   }
   
   private void addMergeSolution(HashSet<Pair<Integer, Integer>> solution, int i, int j) {
      Pair<Integer, Integer> p = new Pair<>(i, j);
      if(solution.contains(p)) {
         solution.remove(p);
      } else {
         solution.add(p);
      }
   }

}
