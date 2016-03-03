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
      new Task1079().run();
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
      //Reader reader = new FileReader("../input.txt");
      //Writer writer = oj ? new OutputStreamWriter(System.out) : new FileWriter("output.txt");\
      Writer writer = new OutputStreamWriter(System.out);
      in = new StreamTokenizer(new BufferedReader(reader));
      out = new PrintWriter(writer);

      solve();
      out.flush();
   }
   int[] a;
   void solve() throws IOException {
      a = new int[100000];
      a[1]=1;
      int n = 0;
      while ((n = nextInt()) > 0) {
         int start = n >> 1;
         if(start % 2 == 0) {
            start -= 1;
         }
         if (start <=0) {
            start = 1;
         }
         int max = 0;
         int next = 0;
         for (int j = start; j <=n; j+=2) {
            next = calcElement(j);
            if (next > max) {
               max = next;
            }
         }
         out.println(max);
      }
   }
   
   int calcElement(int idx) {
      if (a[idx] > 0 || idx == 0) {
         return a[idx];
      }
      int j = idx >> 1;
      if(j<<1 == idx) {
         a[idx] = calcElement(j);
      } else {
         a[idx] = calcElement(j) + calcElement(j+1);
      }
      return a[idx];
   }
   
}
