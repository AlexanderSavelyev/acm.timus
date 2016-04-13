import java.io.*;
import java.util.ArrayList;
import java.util.BitSet;
import java.util.HashMap;
import java.util.HashSet;
import java.util.LinkedList;



public class Task {
   
   class PartHash {
      public HashMap<String, LinkedList<Integer>> elems = new HashMap<>();
   }


   public static void main(String[] args) throws IOException {
      Reader reader = new InputStreamReader(System.in);
      Writer writer = new OutputStreamWriter(System.out);
      new Task().run(reader, writer);
   }
   StreamTokenizer in;
   PrintWriter out;

   int nextInt() throws IOException {
      in.nextToken();
      return (int) in.nval;
   }
   String nextString() throws IOException {
      in.nextToken();
      return in.sval;
   }

   void run(Reader reader, Writer writer) throws IOException {
      in = new StreamTokenizer(new BufferedReader(reader));
      out = new PrintWriter(writer);
      solve();
      out.flush();
   }
   void solve() throws IOException {
      int N = nextInt();
      HashMap<String, Integer> hwords = new HashMap<>();
      ArrayList<String> words = new ArrayList<>();
      for (int i = 0; i < N; i++) {
         String n = nextString();
         hwords.put(n, words.size());
         words.add(n);
      }
      HashMap<Integer, PartHash> partH = new HashMap<>();
      for(String word: words) {
         if(!partH.containsKey(word.length())) {
            
         }
      }
      
//         out.println(1);
   }
   
   
}