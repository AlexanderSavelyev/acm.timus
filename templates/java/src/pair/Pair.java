/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */
package pair;

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
