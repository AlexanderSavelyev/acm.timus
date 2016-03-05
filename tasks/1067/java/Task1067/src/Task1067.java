/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */
import java.io.*;
import java.util.StringTokenizer;
/**
 *
 */
public class Task1067 {

   /**
    * @param args the command line arguments
    */
   public static void main(String[] args) throws IOException {
      new Task1067().run();
   }
   BufferedReader in;
   PrintWriter out;

//   int nextInt() throws IOException {
//      in.nextToken();
//      return (int) in.nval;
//   }
//   
//   String nextLine() throws IOException {
//      in.nextToken();
//      return in.sval;
//   }

   void run() throws IOException {
      boolean oj = System.getProperty("ONLINE_JUDGE") != null;
      Reader reader = oj ? new InputStreamReader(System.in, "ISO-8859-1") : new FileReader("input.txt");
      //Reader reader = new FileReader("../input.txt");
      //Writer writer = oj ? new OutputStreamWriter(System.out) : new FileWriter("output.txt");\
      Writer writer = new OutputStreamWriter(System.out, "ISO-8859-1");
      in = new BufferedReader(reader);
      out = new PrintWriter(writer);

      solve();
      out.flush();
   }
   
   void solve() throws IOException {
      int N = Integer.parseInt(in.readLine());
      for (int i = 0; i < N; i++) {
         out.println(in.readLine());
      }
   }
   
}
