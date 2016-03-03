/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */
package task;

import java.io.*;

/**
 *
 */
public class Task1079 {

   /**
    * @param args the command line arguments
    */
   public static void main(String[] args) throws IOException {
      new Task1079().solve();
   }
   StreamTokenizer in;
   PrintWriter out;

   int nextInt() throws IOException {
      in.nextToken();
      return (int) in.nval;
   }

   void run() throws IOException {
      boolean oj = System.getProperty("ONLINE_JUDGE") != null;
      //Reader reader = oj ? new InputStreamReader(System.in) : new FileReader("../input.txt");
      Reader reader = new FileReader("../input.txt");
      //Writer writer = oj ? new OutputStreamWriter(System.out) : new FileWriter("output.txt");\
      Writer writer =  new OutputStreamWriter(System.out);
      in = new StreamTokenizer(new BufferedReader(reader));
      out = new PrintWriter(writer);

      solve();
      out.flush();
   }

   void solve() throws IOException {
      int a = nextInt();
      int b = nextInt();
      out.print(a + b);
      out.print(" ");
      out.println(a - b);
   }
}
