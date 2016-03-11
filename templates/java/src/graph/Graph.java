/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */
package graph;

import java.util.ArrayList;
import java.util.BitSet;
import java.util.LinkedList;

class Graph {
   class Node {
      public BitSet nei;
      public Node(int N) {
         nei = new BitSet(N);
      }
   }

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
      marked = new boolean[N];
   }

   public boolean insertEdge(int left, int right) {
      if (nodes.get(right).nei.get(left)) {
         return false;
      }
      nodes.get(left).nei.set(right);
      return true;
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

//   private boolean edgeExists(int a, int b) {
//      return nodes.get(a).nei.get(b);
//   }

}
