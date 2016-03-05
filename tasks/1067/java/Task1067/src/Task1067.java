/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */
import java.io.*;
import java.util.Map;
import java.util.StringTokenizer;
import java.util.TreeMap;
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
   class Node {
      public TreeMap<String, Node> nodes = new TreeMap<>();
   }
   void solve() throws IOException {
      int N = Integer.parseInt(in.readLine());
      StringTokenizer tokens;
      Node root = new Node();
      
      for (int i = 0; i < N; i++) {
         tokens = new StringTokenizer(in.readLine(), "\\");
         Node currentNode = root;
         Node parent = null;
         String parentName = null;
         while(tokens.hasMoreTokens()){
            String name = tokens.nextToken();
            if(currentNode == null && parent != null) {
               currentNode = new Node();
               parent.nodes.put(parentName, currentNode);
            }
            if(!currentNode.nodes.containsKey(name)) {
               currentNode.nodes.put(name, null);
            }
            parent = currentNode;
            currentNode = currentNode.nodes.get(name);
            parentName = name;
         }
      }
      printNodes(root, 0);
   }
   void printNodes(Node node, int level) {
      for(Map.Entry<String, Node> kv :node.nodes.entrySet()) {
//         for (int i = 0; i < level; i++) {
//            out.print(" ");
//         }
         if(level > 0) {
            out.format("%" + level + "s", "");
         }
         out.println(kv.getKey());
         if(kv.getValue() != null) {
            printNodes(kv.getValue(), level + 1);
         }
      }
   }
}
