import java.io.*;

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
      Reader reader = oj ? new InputStreamReader(System.in) : new FileReader("../input.txt");
      //Writer writer = oj ? new OutputStreamWriter(System.out) : new FileWriter("output.txt");\
      Writer writer = new OutputStreamWriter(System.out);
      in = new StreamTokenizer(new BufferedReader(reader));
      out = new PrintWriter(writer);

      solve();
      out.flush();
   }
   void solve() throws IOException {
         out.println(1);
   }
   
   
}